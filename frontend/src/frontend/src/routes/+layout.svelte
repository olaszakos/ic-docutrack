<script>
  import Navbar from "$lib/components/Navbar.svelte";
  import "../app.css";
</script>

<div class="pb-20">
  <Navbar />
  <div class="max-w-5xl px-4 mx-auto pt-6">
    <slot />
  </div>
</div>
