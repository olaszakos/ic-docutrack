<script lang="ts">
  import { page } from "$app/stores";
  import { arrayBufferToBase64 } from "$lib/buffer";
  import FilePreview from "$lib/components/FilePreview.svelte";
  import ShareModal from "$lib/components/ShareModal.svelte";
  import BackIcon from "$lib/components/icons/BackIcon.svelte";
  import DownloadIcon from "$lib/components/icons/DownloadIcon.svelte";
  import ShareIcon from "$lib/components/icons/ShareIcon.svelte";
  import { formatUploadDate } from "$lib/shared/dates";
  import File from "$lib/file";
  import { enumIs } from "$lib/shared/enums";
  import { actor, isAuthenticated } from "$lib/shared/stores/auth";
  import { unreachable } from "$lib/shared/unreachable";
  import { onMount } from "svelte";
  import type { file_metadata } from "../../../../declarations/backend/backend.did";

  const fileId = parseInt($page.url.searchParams.get("fileId") || "");

  type State =
    | {
        type: "uninitialized";
      }
    | {
        type: "loading";
      }
    | {
        type: "loaded";
        name: string;
        dataType: string;
        data: string;
        uploadDate: string;
        downloadUrl: string;
        isOpenShareModal: boolean;
        originalMetadata: file_metadata;
      }
    | {
        type: "error";
        error: string;
      };

  let state: State = {
    type: "uninitialized",
  };

  // let errorMessage: string | null = null;
  // let download = "";
  // let loading = true;

  $: {
    if (!$isAuthenticated) {
      state = {
        type: "error",
        error: "User must be authenticated and authorized",
      };
    } else {
    }
  }

  function openShareDialog() {
    if (state.type === "loaded") {
      state = {
        ...state,
        isOpenShareModal: true,
      };
    } else {
      initialize();
    }
  }

  async function initialize() {
    if ($isAuthenticated && $actor) {
      state = {
        type: "loading",
      };

      let files = await $actor.get_requests();
      files = files.concat(await $actor.get_shared_files());
      console.log(files);

      const maybeFile = files.find((entry) => entry.file_id == BigInt(fileId));

      if (!maybeFile) {
        state = {
          type: "error",
          error: "File not found",
        };
        return;
      }

      if (enumIs(maybeFile.file_status, "pending")) {
        state = {
          type: "error",
          error: "File not uploaded",
        };
        return;
      }

      let downloadedFile = await $actor.download_file(BigInt(fileId), 0n);
      console.log(downloadedFile);

      if (enumIs(downloadedFile, "found_file")) {
        for (let i = 1; i < downloadedFile.found_file.num_chunks; i++) {
          console.log("Downloading chunk " + i);

          const downloadedChunk = await $actor.download_file(
            BigInt(fileId),
            BigInt(i)
          );

          if (enumIs(downloadedChunk, "found_file")) {
            const chunk = downloadedChunk.found_file.contents;

            const mergedArray = new Uint8Array(
              downloadedFile.found_file.contents.length + chunk.length
            );
            mergedArray.set(downloadedFile.found_file.contents, 0);
            mergedArray.set(chunk, downloadedFile.found_file.contents.length);

            downloadedFile.found_file.contents = mergedArray;
          } else if (enumIs(downloadedChunk, "not_found_file")) {
            state = {
              type: "error",
              error: "Chunk not found",
            };
            break;
          } else if (enumIs(downloadedChunk, "permission_error")) {
            // errorMessage = "Permission error";
            state = {
              type: "error",
              error: "Permission error",
            };
            break;
          }
        }

        let decryptedFile = await File.fromEncrypted(
          maybeFile.file_name,
          (downloadedFile.found_file.contents as Uint8Array).buffer,
          (downloadedFile.found_file.owner_key as Uint8Array).buffer
        );

        state = {
          type: "loaded",
          data: arrayBufferToBase64(decryptedFile.contents),
          name: decryptedFile.name,
          dataType: downloadedFile.found_file.file_type,
          uploadDate: formatUploadDate(
            maybeFile.file_status.uploaded.uploaded_at
          ),
          downloadUrl: `data:${
            downloadedFile.found_file.file_type
          };base64,${arrayBufferToBase64(decryptedFile.contents)}`,
          isOpenShareModal: false,
          originalMetadata: maybeFile,
        };

        // file.dataType = downloadedFile.found_file.file_type;
        // file.data = arrayBufferToBase64(decryptedFile.contents);
        // download = `data:${file.dataType};base64,${file.data}`;
      } else if (enumIs(downloadedFile, "not_found_file")) {
        // errorMessage = "File not found";
        state = {
          type: "error",
          error: "File not found",
        };
      } else if (enumIs(downloadedFile, "permission_error")) {
        // errorMessage = "Permission error";
        state = {
          type: "error",
          error: "Permission error",
        };
      } else if (enumIs(downloadedFile, "not_uploaded_file")) {
        // errorMessage = "File not uploaded";
        state = {
          type: "error",
          error: "File not uploaded",
        };
      }
    }
  }

  onMount(async () => {
    await initialize();
  });
</script>

<svelte:head>
  <title>DocuTrack: Details</title>
  <meta name="description" content="DocuTrack" />
</svelte:head>
<section>
  <a href="/" class="btn btn-ghost">
    <BackIcon /> Back to files
  </a>
  {#if state.type === "loading" || state.type === "uninitialized"}
    <div class="title-1 mb-2 mt-3 text-text-200">Loading...</div>
  {:else if state.type === "error"}
    <h4>{state.error}</h4>
  {:else if state.type === "loaded"}
    <h1 class="title-1 mb-2 mt-3">
      {#if state.name}
        {state.name}
      {:else}
        <span class="opacity-50">Unnamed file</span>
      {/if}
    </h1>
    <p class="mb-6 text-text-200">Uploaded: {state.uploadDate}</p>
    <div class="mb-6 flex gap-2">
      <a
        href={state.downloadUrl}
        class="btn btn-accent md:w-64"
        download={state.name}
      >
        <DownloadIcon />
        Download</a
      >

      <button class="btn btn-accent md:w-64" on:click={openShareDialog}>
        <ShareIcon /> Share
      </button>
    </div>
    <FilePreview
      file={{
        name: state.name,
        data: state.data,
        dataType: state.dataType,
      }}
    />
    <ShareModal
      bind:isOpen={state.isOpenShareModal}
      bind:fileData={state.originalMetadata}
    />
  {:else}
    {unreachable(state)}
  {/if}
</section>
