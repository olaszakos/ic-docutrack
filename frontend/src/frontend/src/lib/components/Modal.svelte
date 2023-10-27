<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import CloseIcon from "./icons/CloseIcon.svelte";
  import { createEventDispatcher } from "svelte";
  import { mediaQueryStore } from "$lib/shared/stores/media";

  export let isOpen = false;
  export let title: string;

  const dispatch = createEventDispatcher<{
    cancelled: void;
  }>();

  function cancel() {
    isOpen = false;
    dispatch("cancelled");
  }

  $: {
    if (isOpen) {
      document.querySelector("body")!.style.overflow = "hidden";
      document.querySelector("body")!.style.touchAction = "none";
    } else {
      document.querySelector("body")!.style.overflow = "unset";
      document.querySelector("body")!.style.touchAction = "unset";
    }
  }

  const small = mediaQueryStore("(max-width: 767px)");
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 z-20"
    in:fade={{ duration: 100 }}
    out:fade={{ duration: 100 }}
    on:click={cancel}
    aria-label="Close Modal"
    on:keypress={() => {}}
  />

  <div
    class="fixed z-30 bottom-0 left-0 right-0 md:right-auto  md:top-1/2 md:left-1/2  md:-translate-x-1/2 md:-translate-y-1/2"
    transition:fly={$small
      ? { duration: 200, y: 1000 }
      : { y: 0, duration: 200 }}
  >
    <div
      class="bg-background-100 rounded-t-2xl md:panel px-6 pt-4 pb-10 md:pb-3 max-w-none lg:max-w-xl"
    >
      <div class="flex justify-between mb-6">
        <h2 class="text-title-2 font-medium">{title}</h2>
        <button class="" on:click={cancel}>
          <CloseIcon />
        </button>
      </div>
      <slot />
    </div>
  </div>
{/if}
