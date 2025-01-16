<script>
    import Thread from "./ThreadUi.svelte";
    
    export let threads = [];
  </script>
  
  <div>
    {#each threads as thread (thread.hash)}
      <Thread {thread} />
    {/each}
  </div>
  