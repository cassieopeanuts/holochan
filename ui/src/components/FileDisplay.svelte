<script lang="ts">
    import { onMount } from "svelte";
    import  createFileStore  from "../store/FileStore.svelte";
  
    export let client; // Expecting `client` to be passed as a prop
    export let cellIdB64; // Expecting `cellIdB64` to be passed as a prop
  
    let fileStore;
    let downloadedFile;
  
    onMount(() => {
      if (!client || !cellIdB64) {
        throw new Error("Both `client` and `cellIdB64` must be provided to FileDisplay.");
      }
  
      fileStore = new createFileStore(client);
    });
  
    async function downloadFile(entryHashB64) {
      try {
        await fileStore.download(cellIdB64, entryHashB64);
        const fileData = $fileStore[entryHashB64];
        downloadedFile = fileData?.file;
      } catch (e) {
        console.error("Download failed:", e);
      }
    }
  </script>
  
  <button on:click={() => downloadFile("example-entry-hash")}>Download File</button>
  {#if downloadedFile}
    <img src={URL.createObjectURL(downloadedFile)} alt="Downloaded File" />
  {/if}
  