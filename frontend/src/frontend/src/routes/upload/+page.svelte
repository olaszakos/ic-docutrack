<script>
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import FilePreview from "$lib/components/FilePreview.svelte";
  import File from "$lib/file";
  import { actor } from "$lib/shared/stores/auth.js";
  import crypto from "$lib/crypto";

  const alias = $page.url.searchParams.get("alias") || "";

  const chunkSize = 2000000;

  let loading = true;
  let uploadingStatus = "";
  let fileInfo = null;
  let file;
  let files;
  let actorValue;
  let requestStatus = "";
  actor.subscribe(async (value) => {
    actorValue = value;
  });

  onMount(async () => {
    fileInfo = await actorValue.get_alias_info(alias);
    loading = false;
  });

  function onChange() {
    if (files) {
      let inputFile = files[0];

      const reader = new FileReader();

      file = {
        name: inputFile.name,
        dataType: inputFile.type,
        data: "",
      };

      reader.readAsDataURL(inputFile);
      reader.onload = function () {
        let base64 = reader.result;
        let pattern = "base64,";
        let idx = base64.indexOf("base64,");
        file.data = base64.substring(idx + pattern.length);
      };
    }
  }

  const handleUpload = async () => {
    const fileSelector = document.getElementById("file-selector");
    const fileBytes = await fileSelector.files[0].arrayBuffer();
    const fileName = fileInfo?.Ok
      ? fileInfo.Ok.file_name
      : document.getElementById("file-name")?.value;

    let fileToEncrypt = File.fromUnencrypted(fileName, fileBytes);

    const userPublicKey = fileInfo?.Ok
      ? fileInfo.Ok.user.public_key.buffer
      : new Uint8Array(await crypto.getLocalUserPublicKey());

    const encryptedFileKey = await fileToEncrypt.getEncryptedFileKey(
      userPublicKey
    );

    const encFile = await fileToEncrypt.encrypt();
    const content = new Uint8Array(encFile);

    if (content.length > 20 * 1024 * 1024) {
      alert("File can be at most 20MiB.");
      return;
    }

    // Upload file
    uploadingStatus = "Uploading...";

    requestStatus = "Loading";

    // Split file into chunks of 2MB.
    const numChunks = Math.ceil(content.length / chunkSize);
    console.log("num chunks: " + numChunks);

    try {
      if (fileInfo?.Ok) {
        // Upload file for request.
        const res = await actorValue.upload_file({
          file_id: fileInfo.Ok.file_id,
          file_content: content.subarray(0, chunkSize),
          owner_key: new Uint8Array(encryptedFileKey),
          file_type: file.dataType,
          num_chunks: numChunks,
        });

        if ("Ok" in res) {
          for (let i = 1; i < numChunks; i++) {
            // Upload chunks
            console.log("uploading chunk " + i);
            await actorValue.upload_file_continue({
              file_id: fileInfo.Ok.file_id,
              contents: content.subarray(i * chunkSize, (i + 1) * chunkSize),
              chunk_id: i,
            });
          }

          uploadingStatus = "File uploaded successfully.";
          requestStatus = "Uploaded!";
        } else {
          uploadingStatus = "An error occurred. Try again.";
          requestStatus = "";
        }
      } else {
        // Upload file atomically.

        const fileId = await actorValue.upload_file_atomic({
          content: content.subarray(0, chunkSize),
          owner_key: new Uint8Array(encryptedFileKey),
          name: fileName,
          file_type: file.dataType,
          num_chunks: numChunks,
        });
        console.log("Uploaded file with id " + fileId);

        for (let i = 1; i < numChunks; i++) {
          // Upload chunks
          console.log("uploading chunk " + i);
          await actorValue.upload_file_continue({
            file_id: fileId,
            contents: content.subarray(i * chunkSize, (i + 1) * chunkSize),
            chunk_id: i,
          });
        }

        uploadingStatus = "File uploaded successfully.";
        requestStatus = "Uploaded!";
      }
    } catch (err) {
      console.error(err);
      uploadingStatus = "An error occurred. Try again.";
      requestStatus = "";
    }
  };
</script>

<h1>File Upload</h1>
{#if loading}
  <p>Loading...</p>
{:else}
  {#if fileInfo.Ok}
    <p>File name: {fileInfo.Ok.file_name}</p>
  {/if}
  <form class="row g3" on:submit|preventDefault={handleUpload}>
    {#if !fileInfo.Ok}
      <div class="col-auto">
        <label for="file-name">File Name:</label>
        <input name="file-name" type="text" id="file-name" required />
      </div>
    {/if}
    <div class="col-auto">
      <input
        bind:files
        on:change={onChange}
        class="form-control"
        type="file"
        id="file-selector"
        required
      />
    </div>
    <div class="col-auto">
      {#if requestStatus}
        <button class="btn btn-primary" type="submit" disabled
          >{requestStatus}</button
        >
      {:else}
        <button class="btn btn-primary" type="submit">Upload</button>
      {/if}
    </div>
  </form>
  <span>{uploadingStatus}</span>
  <br />
  {#if file && file.data}
    <h4>File Preview</h4>
    <FilePreview {file} />
  {/if}
{/if}
