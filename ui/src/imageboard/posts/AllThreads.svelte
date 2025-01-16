<script lang="ts">
  import { onMount } from "svelte";
  import { threads, loading, error, fetchThreads, listenForThreadSignals } from "../../store/ThreadStore";
  import ThreadDetail from "./ThreadDetail.svelte";

  onMount(async () => {
    await fetchThreads();
    listenForThreadSignals();
  });
</script>

{#if $loading}
  <progress />
{:else if $error}
  <div class="alert">Error fetching the threads: {$error}.</div>
{:else if !$threads.length}
  <div class="alert">No threads found.</div>
{:else}
  <div>
    {#each $threads as hash (hash)}
      <ThreadDetail
        threadHash={hash}
        on:thread-deleted={() => fetchThreads()}
      />
    {/each}
  </div>
{/if}
