<script lang="ts">
  import CreateThreadForm from "../elements/CreateThreadForm.svelte";
  import { holoHashToBase64 } from "../lib/utils"; // Import utility function
  import type { HoloHash } from "@holochain/client";

  export let boards: Array<{ hash: string; name: string }> = [];
  export let onCreateThread: (thread: any) => void;
  export let client: any;
  export let cellIdB64: string;
  export let onFileUploaded: (hash: string) => void;
  export let threads: Array<{
    title: string;
    content: string;
    boardHash: string;
    timestamp: number;
    hash: HoloHash;
  }> = [];

  console.log("Homepage props - client:", client, "cellIdB64:", cellIdB64);

  let showCreateForm = false; // State to toggle thread creation form

  function handleCreateThread(event: CustomEvent) {
    console.log("Thread created:", event.detail);
    onCreateThread(event.detail);
    showCreateForm = false; // Close the form after thread creation
  }
</script>

<section class="homepage">
  <header class="header">
    <h1>HoloChan</h1>
    <button on:click={() => (showCreateForm = !showCreateForm)}>
      {showCreateForm ? "Cancel" : "Create Thread"}
    </button>
  </header>

  <!-- Toggle CreateThreadForm based on showCreateForm -->
  {#if showCreateForm}
    <div class="CreateThreadForm">
      <CreateThreadForm
        boards={boards}
        client={client}
        cellIdB64={cellIdB64}
        onFileUploaded={onFileUploaded}
        on:create-thread={handleCreateThread}
      />
    </div>
  {/if}

  <h2>Threads</h2>
  <!-- Render threads or display a message if none exist -->
  {#if threads.length === 0}
    <p>No threads found. Be the first to create one!</p>
  {:else}
    <div class="threads-grid">
      {#each threads as thread}
        <div class="thread">
          <h3>{thread.title}</h3>
          <p>{thread.content}</p>
          <small>Board: {thread.boardHash || "N/A"}</small>
          <small>Hash: {holoHashToBase64(thread.hash)}</small> <!-- Display hash as Base64 -->
        </div>
      {/each}
    </div>
  {/if}
</section>

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .CreateThreadForm {
    border: 1px solid #ccc;
    padding: 1rem;
    margin-top: 1rem;
  }

  button {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background-color: #0056b3;
  }

  .threads-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .thread {
    border: 1px solid #ddd;
    padding: 1rem;
    border-radius: 8px;
    background-color: #f9f9f9;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .thread h3 {
    margin: 0;
    font-size: 1.2rem;
    color: #333;
  }

  .thread p {
    margin: 0.5rem 0;
    color: #666;
  }

  .thread small {
    color: #999;
  }
</style>
