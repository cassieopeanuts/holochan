<!-- src/pages/HomePage.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import CreateThreadForm from "../elements/CreateThreadForm.svelte";
  import Thread from "../elements/ThreadUi.svelte";

  export let threads = [];
  export let onCreateThread;

  let showCreateForm = false;

  function handleCreateThread(event) {
    const { title, content, timestamp, board } = event.detail;
    onCreateThread({ title, content, timestamp, board });
    showCreateForm = false;
  }
</script>

<section class="homepage">
  <div class="header">
    <h2>Welcome to HoloChan!</h2>
    <button on:click={() => (showCreateForm = !showCreateForm)}>
      {showCreateForm ? "Cancel" : "Create Thread"}
    </button>
  </div>

  {#if showCreateForm}
    <CreateThreadForm on:create-thread={handleCreateThread} />
  {/if}

  {#if threads.length === 0}
    <p class="no-threads">No threads found. Be the first to create one!</p>
  {:else}
    <div class="threads-grid">
      {#each threads as thread (thread.hash)}
        <Thread {thread} />
      {/each}
    </div>
  {/if}
</section>

<style>
  .homepage {
    padding: 1rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .threads-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .no-threads {
    text-align: center;
    color: #555;
  }
</style>
