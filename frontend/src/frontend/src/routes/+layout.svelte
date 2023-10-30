<script lang="ts">
  import Navbar from "$lib/components/Navbar.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import { AuthClient } from "@dfinity/auth-client";
  import { createActor } from "../../../declarations/backend";
  import {
    actor,
    authClient,
    firstName,
    isAuthenticated,
    lastName,
  } from "$lib/shared/stores/auth";

  const title = "DocuTrack – Encrypted document sharing and requesting";
  const description =
    "Effortless document sharing on the Internet Computer. No plugins, no passwords. DocuTrack enables seamless document requests, streamlining interactions for service providers and clients.";
  const image = `https://${
    import.meta.env.VITE_FRONTEND_CANISTER_ID
  }.icp0.io/share.jpg`;
  const url = `https://${
    import.meta.env.VITE_FRONTEND_CANISTER_ID
  }.icp0.io{$page.url.pathname}`;
  const domain = `${import.meta.env.VITE_FRONTEND_CANISTER_ID}.icp0.io`;

  onMount(async () => {
    const auth = await AuthClient.create();
    const isAuth = await auth.isAuthenticated();
    if (isAuth) {
      // Canister IDs are automatically expanded to .env config - see vite.config.ts
      const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
      // We pass the host instead of using a proxy to support NodeJS >= v17 (ViteJS issue: https://github.com/vitejs/vite/issues/4794)
      const host = import.meta.env.VITE_HOST;
      const backend = createActor(canisterId, {
        agentOptions: { host, identity: auth.getIdentity() },
      });
      const record = await backend.who_am_i();
      if ("known_user" in record) {
        firstName.set(record.known_user.first_name);
        lastName.set(record.known_user.last_name);
      }
      actor.set(backend);
    }
    authClient.set(auth);
    isAuthenticated.set(isAuth);
  });
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />

  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:type" content="website" />
  <meta name="og:image" content={image} />
  <meta property="og:url" content={url} />

  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:site" content="@dfinity" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={image} />
  <meta property="twitter:url" content={url} />
  <meta property="twitter:domain" content={domain} />
</svelte:head>

<div class="pb-20">
  <Navbar />
  <div class="max-w-5xl px-4 mx-auto pt-6">
    <slot />
  </div>
</div>
