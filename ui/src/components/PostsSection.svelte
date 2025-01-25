<script lang="ts">
  import { onMount, getContext } from "svelte";
  import type { AgentPubKey, EntryHash } from "@holochain/client";
  
  import { clientContext, type ClientContextValue } from "../contexts";
  import { posts, loading, error, fetchPostsForThread, listenForPostSignals, createPost } from "../store/PostStore";
  import CreatePostForm from "../elements/CreatePostForm.svelte";

  export let author: AgentPubKey;
  export let threadHash: EntryHash;
  export let imageHash: EntryHash | undefined;

  onMount(() => {
    // Now we must pass the same <ClientContextValue> as a generic to getContext
    const { getClient } = getContext<ClientContextValue>(clientContext);
    const client = getClient();
    // If needed, pass the client to your store initialization or do something with it.

    // Then do:
    fetchPostsForThread(threadHash);
    listenForPostSignals();
  });

  function onCreatePost(e) {
    const { content, timestamp } = e.detail;
    createPost(author, threadHash, imageHash, content, timestamp);
  }
</script>

<section class="posts-section">
  <h2>Your Posts</h2>

  <CreatePostForm on:create-post={onCreatePost} />

  {#if $loading}
    <progress />
  {:else if $error}
    <div class="alert">Error loading posts: {$error}</div>
  {:else if !$posts.length}
    <div class="alert">No posts found for this thread.</div>
  {:else}
    {#each $posts as postHash}
      <div>Post Hash: {postHash}</div>
    {/each}
  {/if}
</section>
