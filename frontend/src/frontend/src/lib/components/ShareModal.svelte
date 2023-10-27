<script lang="ts">
  import { default as crypto } from "$lib/crypto";
  import { enumIs } from "$lib/shared/enums";
  import { actor, authClient } from "$lib/shared/stores/auth";
  import { Principal } from "@dfinity/principal";
  import { onMount } from "svelte";
  import type {
    file_metadata,
    user,
  } from "../../../../declarations/backend/backend.did";
  import Modal from "./Modal.svelte";
  import CloseIcon from "./icons/CloseIcon.svelte";

  export let isOpen = false;
  export let fileData: file_metadata;

  let expirationDate = null;
  let loading: boolean = false;
  let users: user[] = [];
  let oldSharedWith: user[] = [];
  let newSharedWith: user[] = [];
  let selectEl: HTMLSelectElement;

  function removeItem(arr, value) {
    var index = arr.indexOf(value);
    if (index > -1) {
      arr.splice(index, 1);
    }
    return arr;
  }

  function addPersonToShare(e: Event) {
    const selectEl = e.target as HTMLSelectElement;

    if (selectEl.value) {
      const principal = Principal.fromText(selectEl.value);

      if (principal === selfPrincipal) {
        return;
      }

      const maybeUser = users.find(
        (obj) => obj.ic_principal.compareTo(principal) === "eq"
      );

      const principalNotYetAdded = !newSharedWith.find(
        (obj) => obj.ic_principal.compareTo(principal) === "eq"
      );
      if (!!maybeUser && principalNotYetAdded) {
        newSharedWith = [...newSharedWith, maybeUser];
      }
    }

    selectEl.value = "";
  }

  function removePersonFromShare(principal) {
    let user = newSharedWith.find(
      (obj) => obj.ic_principal.compareTo(principal) === "eq"
    );
    if (user !== null) {
      newSharedWith = removeItem(newSharedWith, user);
      // Assign to itself for reactivity purposes
      newSharedWith = newSharedWith;
      selectEl.value = "";
    }
  }

  async function saveShare() {
    if (!enumIs(fileData.file_status, "uploaded")) {
      return;
    }

    loading = true;
    // If no expiration date is used, set to -1
    let timestamp = -1;
    if (expirationDate) {
      // The expiration date is saved as timestamp in nanoseconds, convert accordingly
      timestamp = Date.parse(expirationDate) * 1e6;
    }
    const documentKey = await crypto.decryptForUser(
      (fileData.file_status.uploaded.document_key as Uint8Array).buffer
    );
    for (let i = 0; i < newSharedWith.length; i++) {
      const encryptedFileKey = await crypto.encryptForUser(
        documentKey,
        (newSharedWith[i].public_key as Uint8Array).buffer
      );
      // TODO: add expiration date to backend call
      await $actor!.share_file(
        newSharedWith[i].ic_principal,
        fileData.file_id,
        new Uint8Array(encryptedFileKey)
      );
    }
    // Go over all old entries and remove the ones that are no longer in the shared list
    for (let i = 0; i < oldSharedWith.length; i++) {
      let res = newSharedWith.find(
        (obj) =>
          obj.ic_principal.compareTo(oldSharedWith[i].ic_principal) === "eq"
      );
      if (!res) {
        await $actor!.revoke_share(
          oldSharedWith[i].ic_principal,
          fileData.file_id
        );
      }
    }
    // Write back the new state, so the the UI updates
    fileData.shared_with = newSharedWith.slice();
    fileData = fileData;
    isOpen = false;
    loading = false;
  }

  function onOpen(isOpen) {
    if (isOpen) {
      // Keep the old version of the shared users
      oldSharedWith = fileData.shared_with.slice();
      // Copy the array and modify this list with the UI
      newSharedWith = fileData.shared_with.slice();
    }
  }

  // We want to ensure that `oldSharedWith` is only updated at the beginning of a new sharing
  $: onOpen(isOpen);

  $: selfPrincipal = $authClient!.getIdentity().getPrincipal();

  $: availableUsers = users.filter(
    (obj) =>
      obj.ic_principal.compareTo(selfPrincipal) !== "eq" &&
      !newSharedWith.find(
        (obj2) => obj.ic_principal.compareTo(obj2.ic_principal) === "eq"
      )
  );

  onMount(async () => {
    let res = await $actor!.get_users();
    if (enumIs(res, "users")) {
      users = res.users.filter(
        (obj) => obj.ic_principal.compareTo(selfPrincipal) !== "eq"
      );
    } else {
      users = [];
    }
  });
</script>

<div>
  <Modal title={`Share "${fileData.file_name || "Unnamed file"}"`} bind:isOpen>
    <form class="flex flex-col gap-4" on:submit|preventDefault={saveShare}>
      <p class="text-body-1 text-text-100">
        Choose the people that have access to this file.
      </p>

      {#if newSharedWith.length > 0}
        <div class="flex flex-wrap gap-2">
          {#each newSharedWith as user}
            <button
              type="button"
              on:click={() => removePersonFromShare(user.ic_principal)}
              class="rounded-full bg-silver py-1 pl-2 pr-1 flex gap-2 text-body-1 text-text-200"
              >{user.first_name}
              {user.last_name}

              <span
                class="bg-silver-700 rounded-full text-white w-4 h-4 flex items-center justify-center"
              >
                <CloseIcon />
              </span>
            </button>
          {/each}
        </div>
      {/if}

      <div class="">
        <label for="shareWith" class="input-label">Share with</label>
        <select
          class="input disabled:bg-background-300 appearance-none"
          value=""
          id="shareWith"
          bind:this={selectEl}
          on:change={addPersonToShare}
          placeholder="Select a user..."
          disabled={availableUsers && availableUsers.length === 0}
        >
          {#if availableUsers && availableUsers.length > 0}
            <option value="" selected disabled>Select a user...</option>
            {#each availableUsers as user}
              <option value={user.ic_principal}
                >{user.first_name} {user.last_name}</option
              >
            {/each}
          {:else if availableUsers && availableUsers.length === 0}
            <option value="" disabled selected>No users to share with</option>
          {/if}
        </select>
      </div>

      <div>
        <label for="expirationDate" class="input-label"
          >Expiration date (optional)</label
        >
        <input
          type="date"
          class="input"
          id="expirationDate"
          bind:value={expirationDate}
        />
      </div>

      <div class="mt-6">
        <button class="btn btn-accent btn-full" disabled={loading}
          >{#if loading}
            Saving changes...
          {:else}
            Save changes
          {/if}</button
        >
      </div>
    </form>
  </Modal>
</div>
