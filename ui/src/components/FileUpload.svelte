<!-- projects/holochan/ui/src/components/FileUpload.svelte -->

<script lang="ts">
  import { createFileStore } from "../store/FileStore";
  import { createEventDispatcher } from "svelte";
  import { decodeCellIdFromBase64, holoHashToBase64 } from "../lib/utils"; // Import utility functions

  export let client: any = null;
  export let cellIdB64: string | null = null;

  const dispatch = createEventDispatcher<{ "file-uploaded": string }>();

  let fileStore: ReturnType<typeof createFileStore> | null = null;
  let selectedFile: File | null = null;

  $: {
    console.log("FileUpload props - client:", client, "cellIdB64:", cellIdB64);
    if (client && cellIdB64) {
      const cellId = decodeCellIdFromBase64(cellIdB64);
      fileStore = createFileStore(client, cellId); // Pass decoded CellId
      console.log("FileStore initialized:", fileStore);
    } else {
      fileStore = null;
      console.warn("FileStore not initialized: missing client or cellIdB64.");
    }
  }

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      selectedFile = input.files[0];
      console.log("File selected:", selectedFile.name);
    } else {
      selectedFile = null;
      console.log("No file selected");
    }
  }

  async function uploadFile() {
    if (!fileStore || !selectedFile) {
      console.error("Cannot upload: missing FileStore or selected file.");
      return;
    }

    try {
      const fileHash = await fileStore.upload(selectedFile); // HoloHash (Uint8Array)
      console.log("File uploaded successfully:", fileHash);
      const fileHashB64 = holoHashToBase64(fileHash); // Convert to Base64 using utility
      dispatch("file-uploaded", fileHashB64); // Dispatch as Base64 string
    } catch (e) {
      console.error("File upload failed:", e);
    }
  }
</script>

<div>
  <input type="file" on:change={handleFileChange} />
  <button on:click={uploadFile} disabled={!selectedFile}>Upload</button>
</div>
