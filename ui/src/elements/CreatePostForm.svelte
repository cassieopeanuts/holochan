<!-- CreatePostForm.svelte -->
<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let content: string = "";
  export let timestamp: number = Date.now();
  export let isPostValid: boolean = false;

  function handleCreatePost() {
    dispatch("create-post", { content, timestamp });
  }
</script>

<div>
  <h3>Create Post</h3>

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

  <button disabled={!isPostValid} on:click={handleCreatePost}>
    Create Post
  </button>
</div>
