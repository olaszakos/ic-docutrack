<script lang="ts">
  import RegistrationModal from "$lib/components/RegistrationModal.svelte";

  import { createActor } from "../../../../declarations/backend";
  import { AuthClient } from "@dfinity/auth-client";
  import {
    actor,
    authClient,
    firstName,
    lastName,
    isAuthenticated,
  } from "$lib/shared/stores/auth";
  import LogoIcon from "./icons/LogoIcon.svelte";
  import IconFile from "./icons/IconFile.svelte";
  import RequestsIcon from "./icons/RequestsIcon.svelte";
  import LogoutIcon from "./icons/LogoutIcon.svelte";
  import { page } from "$app/stores";
  import UploadIcon from "./icons/UploadIcon.svelte";

  let isOpen = false;
  let isOpenRegistrationModal = false;

  let actorValue;
  let authClientValue;
  let firstNameValue;
  let isAuthenticatedValue;

  actor.subscribe((value) => (actorValue = value));
  authClient.subscribe(async (value) => {
    authClientValue = value;
  });
  isAuthenticated.subscribe((value) => (isAuthenticatedValue = value));
  firstName.subscribe((value) => (firstNameValue = value));

  function handleUpdate(event) {
    isOpen = event.detail.isOpen;
  }

  const handleLogin = async () => {
    try {
      // Canister IDs are automatically expanded to .env config - see vite.config.ts
      const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
      // We pass the host instead of using a proxy to support NodeJS >= v17 (ViteJS issue: https://github.com/vitejs/vite/issues/4794)
      const host = import.meta.env.VITE_HOST;
      // Find out which URL should be used for login.
      const iiUrl = import.meta.env.VITE_II_URL;
      await new Promise((resolve, reject) => {
        authClientValue.login({
          identityProvider: iiUrl,
          onSuccess: resolve,
          onError: reject,
        });
      });
      isAuthenticated.set(await authClientValue.isAuthenticated());
      // Create an actor to interact with the IC for a particular canister ID
      actor.set(
        createActor(canisterId, {
          agentOptions: { host, identity: authClientValue.getIdentity() },
        })
      );
      // Assign to itself for reactivity purposes
      authClient.set(authClientValue);

      let record = await actorValue.who_am_i();

      if ("unknown_user" in record) {
        isOpenRegistrationModal = true;
      } else {
        firstName.set(record.known_user.first_name);
        lastName.set(record.known_user.last_name);
      }
    } catch (err: unknown) {
      console.error(err);
    }
  };

  const handleLogout = async () => {
    await authClientValue.logout();
    isAuthenticated.set(false);
    actor.set(null);
    firstName.set(null);
    lastName.set(null);
    // authClient.set(null);
    authClient.set(await AuthClient.create());
  };
</script>

{#if isAuthenticatedValue !== null}
  <nav class="bg-background-200 rounded-b-3xl">
    <div class="flex h-16 items-center max-w-5xl mx-auto px-4">
      <a href="/">
        <img src="/logo.svg" alt="" />
      </a>
      {#if firstNameValue}
        <div class="flex ml-2">
          <div class="bg-accent-100/10">
            <div class="bg-background-200 w-3 h-full rounded-br-lg" />
          </div>
          <div
            class="bg-accent-100/10 p-2 rounded-lg rounded-bl-none text-accent-100 body-1"
          >
            Hi, {firstNameValue}
          </div>
        </div>
      {/if}
      <div class="flex-1" />
      {#if $isAuthenticated === false}
        <button class="btn btn-accent" on:click={handleLogin}>
          <LogoIcon />
          Login
        </button>
      {:else if $isAuthenticated === true}
        <div class="flex gap-8">
          <a
            href="/"
            class="btn btn-ghost"
            class:btn-ghost-active={$page.route.id === "/"}
          >
            <IconFile />
            Files</a
          >
          <a
            href="/upload"
            class="btn btn-ghost"
            class:btn-ghost-active={$page.route.id === "/upload"}
          >
            <UploadIcon />
            Upload</a
          >
          <a
            href="/requests"
            class="btn btn-ghost"
            class:btn-ghost-active={$page.route.id === "/requests"}
          >
            <RequestsIcon />
            Requests</a
          >
          <button on:click={handleLogout} class="btn btn-ghost">
            <LogoutIcon />
            Logout</button
          >
        </div>
      {/if}
    </div>
  </nav>
{/if}

<RegistrationModal isOpen={isOpenRegistrationModal} />
