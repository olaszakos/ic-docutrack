<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { actor, isAuthenticated } from "$lib/shared/stores/auth";
  import ShareModal from "$lib/components/ShareModal.svelte";
  import type { FileData } from "$lib/shared/types";
  import { enumIs } from "$lib/shared/enums";
  import ShareIcon from "$lib/components/icons/ShareIcon.svelte";
  import { goto } from "$app/navigation";
  import RequestModal from "$lib/components/RequestModal.svelte";
  import PlaceholderLogo from "$lib/components/icons/PlaceholderLogo.svelte";
  import type { file_metadata } from "../../../declarations/backend/backend.did";
  import { formatUploadDate, formatUploadDateShort } from "$lib/dates";

  let data:
    | {
        name: string;
        access: string;
        uploadedAt: string;
        uploadedAtShort: string;
        file_id: bigint;
      }[]
    | null = null;
  let fileData: file_metadata[] = []; //{ file_id: null, file_name: "", shared_with: [] };

  let isOpenShareModal = false;
  let isOpenRequestModal = false;
  let shareFileData: file_metadata | undefined = undefined;

  async function openShareModal(file_id: bigint) {
    if (fileData && fileData.length > 0) {
      shareFileData = fileData.find((obj) => {
        return obj.file_id === file_id;
      });
      if (shareFileData) {
        isOpenShareModal = true;
      }
    }
  }

  async function syncBackend(backend) {
    // data = [];
    // return;

    if (backend) {
      fileData = [
        ...(await $actor!.get_requests()),
        ...(await $actor!.get_shared_files()),
      ];

      let newData: typeof data = [];
      // Prepare data for page template
      for (const file of fileData) {
        if (enumIs(file.file_status, "uploaded")) {
          // Determine the sharing status
          let nShared = file.shared_with ? file.shared_with.length : 0;
          let accessMessage = "";
          switch (nShared) {
            case 0:
              accessMessage = "Only You";
              break;
            case 1:
              accessMessage = "You & 1 other";
              break;
            default:
              accessMessage = "You & " + nShared + " others";
          }
          let detailsLink = new URL($page.url.origin + "/details");
          detailsLink.searchParams.append("fileId", file.file_id.toString());

          newData.push({
            name: file.file_name,
            access: accessMessage,
            uploadedAt: formatUploadDate(file.file_status.uploaded.uploaded_at),
            uploadedAtShort: formatUploadDateShort(
              file.file_status.uploaded.uploaded_at
            ),
            file_id: file.file_id,
          });
        }
      }
      // Assign `data` to itself for reactivity purposes
      data = newData;
    } else {
      data = [];
    }
  }

  // If there is a change to the backend actor, reload the data of the page
  // actor.subscribe(async (value) => {
  // await syncBackend(value);
  // });

  // The vars are not persistent, hence they have to be reloaded `onMount`
  // onMount(async () => {
  // await syncBackend($actor!);
  // });

  $: {
    if ($isAuthenticated && $actor) {
      syncBackend($actor!);
    } else {
      data = [];
    }
  }

  function goToDetails(file_id: bigint) {
    goto(`/details?fileId=${file_id}`);
  }
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="DocuTrack" />
</svelte:head>

<section>
  {#if shareFileData}
    <ShareModal bind:isOpen={isOpenShareModal} bind:fileData={shareFileData} />
  {/if}
  {#if $isAuthenticated === null || data === null}
    <h1 class="title-1">Loading...</h1>
  {:else if $isAuthenticated}
    <div class="flex justify-between items-center mb-6">
      <h1 class="title-1">My Files</h1>
      {#if data && data.length > 0}
        <button
          class="hidden md:inline-block btn btn-accent"
          on:click={() => (isOpenRequestModal = true)}
          >Create new file request</button
        >
      {/if}
    </div>
    <div class="md:hidden fixed bottom-0 left-0 right-0 bg-background-200 p-4">
      <button
        class="btn btn-accent btn-full"
        on:click={() => (isOpenRequestModal = true)}
        >Create new file request</button
      >
    </div>
    <RequestModal bind:isOpen={isOpenRequestModal} />
    {#if data && data.length > 0}
      <div class="hidden md:block bg-background-200 w-full rounded-2xl px-2">
        <table class="table-auto w-full border-spacing-y-2 border-separate">
          <thead class="">
            <tr class="body-2 text-text-200 text-left">
              <th class="body-2 pt-4 pb-2 pl-4">Name</th>
              <th class="body-2 pt-6 pb-2 ">Access</th>
              <th class="body-2 pt-6 pb-2 ">Uploaded at</th>
              <th />
            </tr>
          </thead>
          <tbody class="">
            {#each data as file}
              <tr
                class="hover:drop-shadow-xl cursor-pointer text-text-100"
                on:click={() => goToDetails(file.file_id)}
              >
                <td
                  class="pl-4 bg-background-100 rounded-tl-xl rounded-bl-xl body-1"
                >
                  {#if file.name}
                    {file.name}
                  {:else}
                    <span class="opacity-50">Unnamed file</span>
                  {/if}
                </td>
                <td class="bg-background-100 body-1">{file.access}</td>
                <td class="bg-background-100 body-1">{file.uploadedAt}</td>
                <td
                  class="pr-4 bg-background-100 rounded-tr-xl rounded-br-xl body-1 w-32 text-right h-[52px]"
                >
                  <button
                    on:click|preventDefault|stopPropagation={() =>
                      openShareModal(file.file_id)}
                    class="btn btn-icon"
                  >
                    <ShareIcon />
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      <div class="md:hidden flex flex-col gap-2">
        {#each data as file}
          <a
            class="bg-white rounded-xl py-3 px-4 flex flex-col"
            href="/details?fileId={file.file_id}"
          >
            <div class="flex justify-between items-center mb-3">
              <span class="text-text-100 title-2">
                {#if file.name}
                  {file.name}
                {:else}
                  <span class="opacity-50">Unnamed file</span>
                {/if}
              </span>
              <span>
                <button
                  on:click|preventDefault|stopPropagation={() =>
                    openShareModal(file.file_id)}
                  class="btn btn-icon"
                >
                  <ShareIcon />
                </button>
              </span>
            </div>
            <div class="flex flex-col gap-2">
              <div class="flex justify-between items-center">
                <span class="body-1 text-text-200">Access:</span>
                <span class="body-1 text-text-100">{file.access}</span>
              </div>
              <div class="flex justify-between items-center">
                <span class="body-1 text-text-200">Uploaded at:</span>
                <span class="body-1 text-text-100">{file.uploadedAtShort}</span>
              </div>
            </div>
          </a>
        {/each}
      </div>
    {:else if data && data.length == 0}
      <div
        class="panel pt-10 pb-4 text-center flex flex-col items-center gap-4 mt-6"
      >
        <PlaceholderLogo />
        <h2 class="title-2 text-text-200">
          Even when you have no documents, rest assured, your data is secure.
        </h2>
        <div class="pt-4 pb-8">
          <button
            class="btn btn-accent md:w-96"
            on:click|preventDefault={() => (isOpenRequestModal = true)}
            >Create new file request</button
          >
        </div>
      </div>
    {/if}
  {:else}
    <h1 class="title-1">My Files</h1>

    <div
      class="panel pt-10 pb-4 text-center flex flex-col items-center gap-4 mt-6"
    >
      <PlaceholderLogo />
      <h2 class="title-2 text-text-200">Please log in to see your data.</h2>
    </div>
  {/if}
</section>
