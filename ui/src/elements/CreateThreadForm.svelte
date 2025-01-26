<!-- projects/holochan/ui/src/elements/CreateThreadForm.svelte -->

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import FileUpload from "../components/FileUpload.svelte";
  import { base64ToHoloHash } from "../lib/utils"; // Import utility function
  import type { HoloHash } from "@holochain/client";

  const dispatch = createEventDispatcher();

  export let title: string = "";
  export let content: string = "";
  export let boardHash: string | undefined = undefined;
  export let boards: Array<{ hash: string; name: string }> = []; // Define boards as an array of objects

  export let client: any;
  export let cellIdB64: string;
  export let onFileUploaded: (hash: string) => void;

  let fileHash: HoloHash | null = null;

  function handleCreateThread() {
    if (!fileHash) {
      console.error("Cannot create thread: No file uploaded.");
      return;
    }

    dispatch("create-thread", {
      title,
      content,
      boardHash,
      timestamp: Date.now(),
      fileHash, // Uint8Array
    });
  }

  function handleFileUploaded(event: CustomEvent<string>) {
    fileHash = base64ToHoloHash(event.detail); // Convert Base64 string to Uint8Array
    if (onFileUploaded) onFileUploaded(event.detail);
  }
</script>

<FileUpload {client} {cellIdB64} on:file-uploaded={handleFileUploaded} />

<div>
  <h3>Create Thread</h3>

  <div>
    <label for="Title">Title</label>
    <textarea name="Title" bind:value={title} required />
  </div>

  <div>
    <label for="Content">Content</label>
    <textarea name="Content" bind:value={content} required />
  </div>

  <div>
    <label for="Board">Board</label>
    <select bind:value={boardHash} required>
      <option value="" disabled selected>Select a board</option>
      {#each boards as board}
        <option value={board.hash}>{board.name}</option>
      {/each}
    </select>
  </div>

  <button disabled={!title || !content || !boardHash || !fileHash} on:click={handleCreateThread}>
    Create Thread
  </button>
</div>
