<script>
    export let content = "";
    export let timestamp = Date.now();
    export let isPostValid = false;
    export let onCreatePost;
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
        value={new Date(timestamp / 1000 - (new Date(timestamp / 1000).getTimezoneOffset() * 60000)).toISOString().slice(0, 16)}
        on:input={(e) => timestamp = Math.floor(new Date(e.currentTarget.value).getTime() / 1000)}
        required
      />
    </div>
  
    <button disabled={!isPostValid} on:click={() => onCreatePost?.()}>
      Create Post
    </button>
  </div>
  