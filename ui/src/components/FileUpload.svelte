<script lang="ts">
  import { onMount } from "svelte";
  import  createFileStore  from "../store/FileStore.svelte";

  export let client; // Expecting `client` to be passed as a prop
  export let cellIdB64; // Expecting `cellIdB64` to be passed as a prop

  let fileStore;
  let selectedFile;

  onMount(() => {
    if (!client || !cellIdB64) {
      throw new Error("Both `client` and `cellIdB64` must be provided to FileUpload.");
    }

    fileStore = new createFileStore(client);
  });

  async function uploadFile() {
    if (selectedFile) {
      try {
        const fileHash = await fileStore.upload(cellIdB64, selectedFile);
        console.log("Uploaded file hash:", fileHash);
      } catch (e) {
        console.error("Upload failed:", e);
      }
    }
  }
</script>

<input type="file" bind:this={selectedFile} />
<button on:click={uploadFile}>Upload</button>
