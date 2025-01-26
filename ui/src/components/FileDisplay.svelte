<script lang="ts">
  import { onMount } from "svelte";
  import { createFileStore } from "../store/FileStore";

  export let client = null;
  export let cellIdB64 = null;
  export let entryHashB64 = null;

  let fileStore;
  let downloadedFile = null;

  onMount(() => {
    if (!client || !cellIdB64) {
      console.warn("`client` or `cellIdB64` is not ready yet.");
      return;
    }

    fileStore = createFileStore(client, cellIdB64);
  });

  async function downloadFile() {
    if (!fileStore || !entryHashB64) {
      console.error("Cannot download file: FileStore or entryHashB64 is not ready.");
      return;
    }

    try {
      await fileStore.download(cellIdB64, entryHashB64);
      downloadedFile = fileStore.data[entryHashB64]?.file;
    } catch (e) {
      console.error("Download failed:", e);
    }
  }
</script>

<div>
  <button on:click={downloadFile}>Download File</button>
  {#if downloadedFile}
    <img src={URL.createObjectURL(downloadedFile)} alt="Downloaded File" />
  {/if}
</div>
