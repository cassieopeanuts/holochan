<script lang="ts">
  import { onMount, getContext } from "svelte";
  import type { AgentPubKey, HoloHash } from "@holochain/client";
  import { createThreadStore } from "../store/ThreadStore";
  import { clientContext, type ClientContextValue } from "../contexts";

  import CreateThreadForm from "../elements/CreateThreadForm.svelte";
  import ThreadList from "../elements/ThreadList.svelte";

  export let author: AgentPubKey;
  export let imageHash: HoloHash | undefined;

  let threadStore;
  let threads;
  let loading;
  let error;

  onMount(() => {
    // Retrieve the Holochain client from context
    const { getClient } = getContext<ClientContextValue>(clientContext);
    const client = getClient();

    // Pass the client to createThreadStore
    threadStore = createThreadStore(client);

    // Subscribe to store values
    threadStore.threads.subscribe(val => ($threads = val));
    threadStore.loading.subscribe(val => ($loading = val));
    threadStore.error.subscribe(val => ($error = val));

    // Fetch threads and listen for signals
    threadStore.fetchThreads();
    threadStore.listenForSignals();
  });

  async function onCreateThread(e) {
    const { title, content, timestamp, board } = e.detail;
    await threadStore.createThread(author, title, content, timestamp, board, imageHash);
  }

  function refreshThreads() {
    threadStore.fetchThreads();
  }
</script>

<section class="threads-section">
  <h2>Your Threads</h2>

  <CreateThreadForm on:create-thread={onCreateThread} />

  {#if $loading}
    <progress />
  {:else if $error}
    <p class="error">{$error}</p>
  {:else}
    <ThreadList
      loading={$loading}
      error={$error}
      hashes={$threads}
      onThreadDeleted={refreshThreads}
    />
  {/if}
</section>
