<script>
    export let title = "";
    export let content = "";
    export let timestamp = Date.now();
    export let board = "";
    export let isThreadValid = false;
    export let onCreateThread;
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
        value={new Date(timestamp / 1000 - (new Date(timestamp / 1000).getTimezoneOffset() * 60000)).toISOString().slice(0, 16)}
        on:input={(e) => timestamp = Math.floor(new Date(e.currentTarget.value).getTime() / 1000)}
        required
      />
    </div>
    <div>
      <label for="Board">Board</label>
      <input name="Board" bind:value={board} required />
    </div>
  
    <button disabled={!isThreadValid} on:click={() => onCreateThread?.()}>
      Create Thread
    </button>
  </div>
  