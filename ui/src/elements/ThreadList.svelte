<script lang="ts">
    import ThreadDetail from "../imageboard/posts/ThreadDetail.svelte";
    import type { HolochainError, HoloHash } from "@holochain/client";
  
    export let loading: boolean = false;
    export let error: HolochainError | undefined = undefined;
    export let hashes: HoloHash[] = []; // Correctly typed as HoloHash[]
    export let onThreadDeleted: (() => void) | undefined = undefined;
  </script>  
  
  {#if loading}
    <progress />
  {:else if error}
    <div class="alert">Error fetching the threads: {error.message}.</div>
  {:else if !hashes.length}
    <div class="alert">No threads found.</div>
  {:else}
    <div>
      {#each hashes as hash}
        <ThreadDetail
          threadHash={hash}
          on:thread-deleted={() => onThreadDeleted?.()}
        />
      {/each}
    </div>
  {/if}
  