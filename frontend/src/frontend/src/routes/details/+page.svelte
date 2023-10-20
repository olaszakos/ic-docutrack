<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import FilePreview from "$lib/components/FilePreview.svelte";
  import File from "$lib/file";
  import { Alert } from "sveltestrap";
  import Details from "$lib/components/Details.svelte";
  import { actor, isAuthenticated } from "$lib/shared/stores/auth";
  import { arrayBufferToBase64 } from "$lib/buffer";
  import type { ActorType } from "$lib/shared/actor";
  import { enumIs } from "$lib/shared/enums";

  const fileId = parseInt($page.url.searchParams.get("fileId") || "");

  let actorValue: ActorType | null = null;
  let isAuthenticatedValue;
  let errorMessage: string | null = null;
  let download = "";
  let file = {
    name: "",
    dataType: "",
    data: "",
  };
  let loading = true;

  actor.subscribe((value) => (actorValue = value));
  isAuthenticated.subscribe((value) => (isAuthenticatedValue = value));

  onMount(async () => {
    if (isAuthenticatedValue && actorValue) {
      let files = await actorValue.get_requests();
      files = files.concat(await actorValue.get_shared_files());
      console.log(files);

      const maybeFile = files.find((entry) => entry.file_id == BigInt(fileId));

      if (maybeFile) {
        console.log(maybeFile);

        errorMessage = null;
        file.name = maybeFile.file_name;
      } else {
        errorMessage = "File not found";
        return;
      }

      let downloadedFile = await actorValue.download_file(BigInt(fileId), 0n);
      console.log(downloadedFile);

      if (enumIs(downloadedFile, "found_file")) {
        for (let i = 1; i < downloadedFile.found_file.num_chunks; i++) {
          console.log("Downloading chunk " + i);

          const downloadedChunk = await actorValue.download_file(
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
            errorMessage = "Chunk not found";
            break;
          } else if (enumIs(downloadedChunk, "permission_error")) {
            errorMessage = "Permission error";
            break;
          }
        }

        let decryptedFile = await File.fromEncrypted(
          file.name,
          (downloadedFile.found_file.contents as Uint8Array).buffer,
          (downloadedFile.found_file.owner_key as Uint8Array).buffer
        );
        file.dataType = downloadedFile.found_file.file_type;
        file.data = arrayBufferToBase64(decryptedFile.contents);
        download = `data:${file.dataType};base64,${file.data}`;
      } else if (enumIs(downloadedFile, "not_found_file")) {
        errorMessage = "File not found";
      } else if (enumIs(downloadedFile, "permission_error")) {
        errorMessage = "Permission error";
      } else if (enumIs(downloadedFile, "not_uploaded_file")) {
        errorMessage = "File not uploaded";
      }
    }
    loading = false;
  });
</script>

<svelte:head>
  <title>DocuTrack: Details</title>
  <meta name="description" content="DocuTrack" />
</svelte:head>
<section>
  {#if loading}
    <h3>Loading...</h3>
  {:else}
    <h1>Details</h1>
    {#if isAuthenticated && !errorMessage}
      <Details {file} />
      {#if file && file.data}
        <h4>File Preview</h4>
        <a class="btn btn-primary" href={download} download={file.name}
          >Download</a
        >
        <p />
        <FilePreview {file} />
      {/if}
    {:else if errorMessage}
      <h4>{errorMessage}</h4>
    {:else}
      <Alert color="warning">
        <h4 class="alert-heading text-capitalize">warning</h4>
        User must be authenticated and authorized.
      </Alert>
    {/if}
  {/if}
</section>
