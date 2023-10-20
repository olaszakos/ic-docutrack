<script lang="ts">
  import { fade } from "svelte/transition";
  import CloseIcon from "./icons/CloseIcon.svelte";
  import { createEventDispatcher } from "svelte";

  export let isOpen = false;
  export let title: string;

  const dispatch = createEventDispatcher<{
    cancelled: void;
  }>();

  function cancel() {
    isOpen = false;
    dispatch("cancelled");
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/50 z-10"
    in:fade={{ duration: 100 }}
    out:fade={{ duration: 100 }}
    on:click={cancel}
  />

  <div
    class="fixed z-20 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
    in:fade={{ duration: 100 }}
    out:fade={{ duration: 100 }}
  >
    <div class="panel px-6 pt-4 pb-3 max-w-xl">
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
