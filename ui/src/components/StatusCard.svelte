<!-- src/components/StatusCard.svelte -->
<script>
    export let loading;
    export let error;
  </script>
  
  <div class="status-card">
    {#if loading}
      <p>Connecting...</p>
    {:else if error}
      <p class="error">{error}</p>
    {:else}
      <p>Client is connected.</p>
    {/if}
  </div>
  
  <style>
  /* Styles from App.svelte's .status-card */
  </style>
  