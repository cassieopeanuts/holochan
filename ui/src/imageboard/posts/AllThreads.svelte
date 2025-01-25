<script lang="ts">
  import { onMount, getContext } from "svelte";
  import { createThreadStore } from "../../store/ThreadStore";
  import type { AppClient } from "@holochain/client";

  // Retrieve the Holochain client from context
  const client: AppClient = getContext("client");

  // Pass the client to createThreadStore
  const { threads, loading, error, fetchThreads, listenForSignals } = createThreadStore(client);

  onMount(() => {
    fetchThreads();
    listenForSignals();
  });
</script>

{#if $loading}
  <progress />
{:else if $error}
  <p style="color: red;">Error: {$error}</p>
{:else}
  {#each $threads as t}
    <!-- show a thread hash -->
    <div>{t}</div>
  {/each}
{/if}
