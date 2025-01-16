<script lang="ts">
  import { onMount } from "svelte";
  import { posts, loading, error, fetchPostsForThread, listenForPostSignals } from "../store/PostStore";
  import CreatePost from "../imageboard/posts/CreatePost.svelte";
  import AllPosts from "../imageboard/posts/AllPosts.svelte";

  export let author;
  export let threadHash;
  export let imageHash;

  onMount(async () => {
    await fetchPostsForThread(threadHash);
    listenForPostSignals();
  });
</script>

<section class="posts-section">
  <h2>Your Posts</h2>
  <CreatePost {author} {threadHash} {imageHash} />

  {#if $loading}
    <progress />
  {:else if $error}
    <div class="alert">Error loading posts: {$error}</div>
  {:else if !$posts.length}
    <div class="alert">No posts found for this thread.</div>
  {:else}
    <AllPosts {author} {threadHash} />
  {/if}
</section>

<style>
/* Posts section styles from App.svelte */
</style>
