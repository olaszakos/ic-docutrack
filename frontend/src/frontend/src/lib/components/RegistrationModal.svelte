<script lang="ts">
  // import { Modal, ModalBody, ModalHeader } from "sveltestrap";
  import {
    actor,
    authClient,
    firstName,
    lastName,
  } from "$lib/shared/stores/auth";
  import Modal from "./Modal.svelte";
  import { default as crypto } from "$lib/crypto";
  import type { ActorType } from "$lib/shared/actor";
  import type { AuthClient } from "@dfinity/auth-client";

  export let isOpen = false;
  let actorValue: ActorType | null;
  let authClientValue: AuthClient | null;
  let loading = false;

  actor.subscribe((value) => (actorValue = value));
  authClient.subscribe((value) => (authClientValue = value));

  async function setUser(e) {
    loading = true;
    const formData = new FormData(e.target);
    const data: any = {};
    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }
    await actorValue!.set_user(
      data.firstName,
      data.lastName,
      new Uint8Array(await crypto.getLocalUserPublicKey())
    );
    firstName.set(data.firstName);
    lastName.set(data.lastName);
    isOpen = false;
    loading = false;
  }
</script>

<div>
  <Modal {isOpen} title="Register Yourself">
    <form class="" on:submit|preventDefault={setUser}>
      <p class="body-1 text-text-200 mb-4">
        Your Internet Identity is not connected with a name yet. Enter your name
        to setup an account on DocuTrack.
      </p>
      <div class="mb-4">
        <label for="firstName" class="input-label">First Name</label>
        <input
          type="text"
          required={true}
          class="input"
          id="firstName"
          name="firstName"
          placeholder="First Name"
        />
      </div>
      <div class="mb-4">
        <label for="lastName" class="input-label">Last Name</label>
        <input
          type="text"
          required={true}
          class="input"
          id="lastName"
          name="lastName"
          placeholder="Last Name"
        />
      </div>
      <div class="mt-10">
        {#if loading}
          <button type="submit" class="btn btn-full btn-accent" disabled
            >Loading...</button
          >
        {:else}
          <button type="submit" class="btn btn-full btn-accent">Submit</button>
        {/if}
      </div>
    </form>
  </Modal>
</div>
