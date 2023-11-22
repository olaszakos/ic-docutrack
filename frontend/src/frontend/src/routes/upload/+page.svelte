<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import FilePreview from "$lib/components/FilePreview.svelte";
  import File from "$lib/file";
  import crypto from "$lib/crypto";
  import { enumIs } from "$lib/shared/enums";
  import { unreachable } from "$lib/shared/unreachable";
  import type { get_alias_info_response } from "../../../../declarations/backend/backend.did";
  import { actor, isAuthenticated } from "$lib/shared/stores/auth.js";
  import { goto } from "$app/navigation";
  import pLimit from 'p-limit';

  const chunkSize = 2000000;

  type State =
    | {
        state: "uninitialized";
      }
    | {
        state: "upload";
        uploadState:
          | { type: "not-uploaded" }
          | { type: "uploading" }
          | { type: "uploaded" }
          | { type: "error"; message: string };
        filePreview: {
          name: string;
          dataType: string;
          data: string;
        } | null;
        uploadType:
          | {
              type: "request";
              fileInfo: Extract<get_alias_info_response, { Ok: any }>["Ok"];
            }
          | {
              type: "self";
              fileName: string;
            };
      }
    | {
        state: "error";
        message: string;
      };

  let state: State = { state: "uninitialized" };

  /**
   * Uploads all chunks of a file, with the exception of the first chunk.
   * The first chunk is uploaded in a special way and is done separately
   * prior to calling this method.
   * 
   * @param content
   * @param fileId
   */
  async function uploadChunks(content: Uint8Array, fileId: bigint) {
    const numChunks = Math.ceil(content.length / chunkSize);

    // Create upload pool, supporting upto 5 parallel uploads.
    const uploadPool = pLimit(5);

    // Prepare upload requests.
    const uploadRequests = Array.from({ length: numChunks - 1 }, (_, i) => i + 1)
      .map((i) => uploadPool(async () => {
        console.log(`Uploading chunk ${i}`);
        await $actor!.upload_file_continue({
          file_id: fileId,
          contents: content.subarray(i * chunkSize, (i + 1) * chunkSize),
          chunk_id: BigInt(i),
        });
        console.log(`Uploaded chunk ${i}`);
      }));

    await Promise.all(uploadRequests);
    console.log("File upload complete");
  }

      
  function getAlias() {
    return  $page.url.searchParams.get("alias") || "";
  }

  
  let files: FileList | null = null;

  $: {
    if ($isAuthenticated === false && !getAlias()) {
      goto("/");
    }
  }

  onMount(async () => {
    if (!getAlias()) {
      if ($isAuthenticated) {
        state = {
          state: "upload",
          uploadState: { type: "not-uploaded" },
          filePreview: null,
          uploadType: {
            type: "self",
            fileName: "",
          },
        };
      } else {
        goto("/");
      }
    } else {
      const fileInfo = await $actor!.get_alias_info(getAlias());

      if (enumIs(fileInfo, "Ok")) {
        state = {
          state: "upload",
          uploadState: { type: "not-uploaded" },
          filePreview: null,
          uploadType: {
            type: "request",
            fileInfo: fileInfo.Ok,
          },
        };
      } else if (enumIs(fileInfo, "Err")) {
        state = {
          state: "error",
          message: "Unknown alias or file already uploaded.",
        };
      } else {
        unreachable(fileInfo);
      }
    }
  });

  function onChange() {
    if (
      files &&
      state.state === "upload" &&
      state.uploadState.type === "not-uploaded"
    ) {
      let inputFile = files[0];

      const reader = new FileReader();
      const uploadType = state.uploadType;

      reader.readAsDataURL(inputFile);
      reader.onload = function () {
        const base64 = reader.result as string;
        const pattern = "base64,";
        const idx = base64.indexOf(pattern);
        state = {
          state: "upload",
          uploadState: { type: "not-uploaded" },
          filePreview: {
            name: inputFile.name,
            dataType: inputFile.type,
            data: base64.substring(idx + pattern.length),
          },
          uploadType,
        };
      };
    }
  }

  const handleUpload = async () => {
    if (
      state.state !== "upload" ||
      state.uploadState.type !== "not-uploaded" ||
      state.filePreview === null ||
      files === null ||
      files.length === 0
    ) {
      return;
    }

    state = {
      ...state,
      uploadState: { type: "uploading" },
    };
    const userPublicKey =
      state.uploadType.type === "request"
        ? (state.uploadType.fileInfo.user.public_key as Uint8Array).buffer
        : new Uint8Array(await crypto.getLocalUserPublicKey());

    const fileName =
      state.uploadType.type === "request"
        ? state.uploadType.fileInfo.file_name
        : state.uploadType.fileName;

    const fileBytes = await files![0].arrayBuffer();
    let fileToEncrypt = File.fromUnencrypted(fileName, fileBytes);
    const encryptedFileKey = await fileToEncrypt.getEncryptedFileKey(
      userPublicKey
    );

    const encFile = await fileToEncrypt.encrypt();
    const content = new Uint8Array(encFile);

    if (content.length > 20 * 1024 * 1024) {
      alert("File size is limited to 20MiB in this PoC\n(larger files could be supported in a production version).");
      return;
    }

    // Split file into chunks of 2MB.
    const numChunks = Math.ceil(content.length / chunkSize);
    console.log("num chunks: " + numChunks);

    try {
      if (state.uploadType.type === "request") {
        // Upload file for request.
        const res = await $actor!.upload_file({
          file_id: state.uploadType.fileInfo.file_id,
          file_content: content.subarray(0, chunkSize),
          owner_key: new Uint8Array(encryptedFileKey),
          file_type: state.filePreview!.dataType,
          num_chunks: BigInt(numChunks),
        });

        if (enumIs(res, "Ok")) {
          await uploadChunks(content, state.uploadType.fileInfo.file_id);

          state = {
            ...state,
            uploadState: { type: "uploaded" },
          };
        } else {
          state = {
            ...state,
            uploadState: {
              type: "error",
              message:
                "An error occurred while uploading the file. Please try again.",
            },
          };
        }
      } else {
        // Upload file atomically.

        const fileId = await $actor!.upload_file_atomic({
          content: content.subarray(0, chunkSize),
          owner_key: new Uint8Array(encryptedFileKey),
          name: fileName,
          file_type: state.filePreview!.dataType,
          num_chunks: BigInt(numChunks),
        });
        console.log("Uploaded file with id " + fileId);

        await uploadChunks(content, fileId);

        state = {
          ...state,
          uploadState: { type: "uploaded" },
        };
      }
    } catch (err) {
      console.error(err);
      state = {
        ...state,
        uploadState: {
          type: "error",
          message:
            "An error occurred while uploading the file. Please try again.",
        },
      };
    }
    // const res = await $actor!.upload_file({
    //   file_id: state.fileInfo.file_id,
    //   file_content: new Uint8Array(encFile),
    //   owner_key: new Uint8Array(encryptedFileKey),
    //   file_type: state.filePreview!.dataType,
    // });

    // if (enumIs(res, "Ok")) {
    //   state = {
    //     ...state,
    //     uploadState: "uploaded",
    //   };
    // } else {
    //   state = {
    //     ...state,
    //     uploadState: "error",
    //   };
    // }
  };
</script>

<div class="flex justify-start items-center gap-7 mb-2">
  <h1 class="title-1">File Upload</h1>
  {#if state.state === "upload" && state.uploadState.type === "uploaded"}
    <span class="body-1 px-2 py-1 bg-success text-white rounded-full">
      Uploaded
    </span>
  {/if}
</div>
{#if state.state === "uninitialized"}
  <p>Loading...</p>
{:else if state.state === "error"}
  <p>{state.message}</p>
{:else if state.state === "upload"}
  {#if state.uploadType.type === "request"}
    <p class="body-1 text-text-200 mb-6">
      Request: {state.uploadType.fileInfo.file_name}
    </p>
  {:else if state.uploadType.type === "self"}
    <!--  -->
  {:else}
    {unreachable(state.uploadType)}
  {/if}

  {#if ["not-uploaded", "uploading", "error"].includes(state.uploadState.type)}
    <form
      class="flex flex-col gap-4 max-w-lg mb-10"
      on:submit|preventDefault={handleUpload}
    >
      {#if state.uploadType.type === "self"}
        <div class="">
          <label for="fileName" class="input-label">File Name</label>
          <input
            type="text"
            required={true}
            class="input"
            id="fileName"
            name="fileName"
            placeholder="File name"
            bind:value={state.uploadType.fileName}
          />
        </div>
      {/if}
      <div class="">
        <label for="fileNfile-selectorame" class="input-label"
          >File to upload</label
        >
        <input
          bind:files
          on:change={onChange}
          id="file-selector"
          required
          class="
          block
          w-full
          file:cursor-pointer
          file:py-2 file:px-4 
          file:body-1 file:text-text-100 
          file:rounded-full file:bg-silver file:border-none 
          file:mr-2
          hover:file:brightness-110
          p-2
          bg-background-100 rounded-md 
          border-silver border-solid border-[1.5px]
          text-text-100
          body-1"
          type="file"
        />
      </div>
      <div class="pt-3">
        {#if state.uploadState.type === "not-uploaded" || state.uploadState.type === "error"}
          <button class="btn btn-accent" type="submit">Upload file</button>
        {:else if state.uploadState.type === "uploading"}
          <button class="btn btn-accent" type="submit" disabled
            >Uploading...</button
          >
        {/if}
      </div>
    </form>
  {/if}
  {#if state.filePreview}
    <FilePreview file={state.filePreview} />
  {/if}
{:else}
  {unreachable(state)}
{/if}
