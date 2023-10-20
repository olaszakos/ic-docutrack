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

  let data:
    | {
        name: string;
        access: string;
        uploadedAt: string;
        file_id: string;
        // items: (
        //   | { url: URL; text: string }
        //   | {
        //       onClick: () => void;
        //       text: string;
        //     }
        // )[];
      }[]
    | null = null;
  let fileData: FileData[] = []; //{ file_id: null, file_name: "", shared_with: [] };

  let isOpenShareModal = false;
  let isOpenRequestModal = false;
  let shareFileData: FileData | undefined = undefined;
  let actorValue;
  let isAuthenticatedValue;

  actor.subscribe((value) => {
    actorValue = value;
  });
  isAuthenticated.subscribe((value) => {
    isAuthenticatedValue = value;
  });

  async function openShareModal(file_id) {
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
        ...(await actorValue.get_requests()),
        ...(await actorValue.get_shared_files()),
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
          detailsLink.searchParams.append("fileId", file.file_id);
          const dateOptions: Intl.DateTimeFormatOptions = {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
            timeZone: "CET",
            hour12: false,
          };
          let uploadedAt = new Date(
            Math.floor(Number(file.file_status.uploaded.uploaded_at) / 1000000)
          );
          newData.push({
            name: file.file_name,
            access: accessMessage,
            uploadedAt: uploadedAt.toLocaleTimeString("en-CH", dateOptions),
            file_id: file.file_id,
            // items: [
            //   { url: detailsLink, text: "Open" },
            //   {
            //     onClick: () => {
            //       openShareModal(file.file_id);
            //     },
            //     text: "Share",
            //   },
            // ],
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
  onMount(async () => {
    await syncBackend(actorValue);
  });

  function goToDetails(file_id: string) {
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
  {#if isAuthenticatedValue === null || data === null}
    <h1 class="title-1">Loading...</h1>
  {:else if isAuthenticatedValue}
    <div class="flex justify-between items-center mb-6">
      <h1 class="title-1">My Files</h1>
      {#if data && data.length > 0}
        <button
          class="btn btn-accent"
          on:click={() => (isOpenRequestModal = true)}
          >Create new file request</button
        >
      {/if}
    </div>
    <RequestModal bind:isOpen={isOpenRequestModal} />
    {#if data && data.length > 0}
      <div class="bg-background-200 w-full rounded-2xl px-2">
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
                  >{file.name}</td
                >
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
                  <!-- <a href="/details?fileId={file.file_id}" class="btn btn-icon">
                  <DownloadIcon />
                </a> -->
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
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
            on:click={() => (isOpenRequestModal = true)}
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
