<!-- CreateThreadForm.svelte -->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let title: string = "";
  export let content: string = "";
  export let timestamp: number = Date.now();
  export let board: string = "";
  export let isThreadValid: boolean = false;

  function handleCreateThread() {
    dispatch("create-thread", { title, content, timestamp, board });
  }
</script>

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
    <label for="Timestamp">Timestamp</label>
    <input
      name="Timestamp"
      type="datetime-local"
      bind:value={timestamp}
      required
    />
  </div>

  <div>
    <label for="Board">Board</label>
    <input name="Board" bind:value={board} required />
  </div>

  <button disabled={!isThreadValid} on:click={handleCreateThread}>
    Create Thread
  </button>
</div>
