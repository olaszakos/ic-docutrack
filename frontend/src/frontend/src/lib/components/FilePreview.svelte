<script>
  export let file;
  let previewStyle;
  let previewClass;

  const supportedDataTypes = [
    "application/pdf",
    "image/jpg",
    "image/jpeg",
    "image/png",
    "image/webp",
  ];
  $: previewStyle =
    file.dataType === "application/pdf" ? "--bs-aspect-ratio: 141%" : "";
  $: previewClass = file.dataType === "application/pdf" ? "ratio" : "";
</script>

<div class="flex items-center mb-3 {previewClass}" style={previewStyle}>
  {#if supportedDataTypes.includes(file.dataType)}
    <embed src="data:{file.dataType};base64,{file.data}" class="max-w-full" />
  {:else}
    <p class="mt-10 text-text-200 body-1">No preview available.</p>
  {/if}
</div>
