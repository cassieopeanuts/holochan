<!-- src/elements/ThreadUi.svelte -->
<script lang="ts">
  export let thread;
</script>

<div class="thread-preview">
  <h4>{thread.title}</h4>
  <p>{thread.content.slice(0, 100)}{thread.content.length > 100 ? "..." : ""}</p>
  <small>Posted on {new Date(thread.timestamp).toLocaleString()}</small>
</div>

<style>
  .thread-preview {
    border: 1px solid #ddd;
    padding: 1rem;
    border-radius: 8px;
    background: #f9f9f9;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .thread-preview h4 {
    margin: 0 0 0.5rem;
    font-size: 1.2rem;
  }

  .thread-preview p {
    margin: 0 0 0.5rem;
    color: #666;
  }

  .thread-preview small {
    color: #999;
  }
</style>
