<script lang="ts">
  import logo from "./assets/holo_chan.png";
  import Navbar from "./components/Navbar.svelte";
  import ClientProvider from "./components/ClientProvider.svelte";
  import StatusCard from "./components/StatusCard.svelte";
  import ThreadsSection from "./components/ThreadSection.svelte";
  import PostsSection from "./components/ThreadSection.svelte";

  let author;
  let yourImageHash = "example-image-hash";
  let yourThreadHash = "example-thread-hash";
</script>

<ClientProvider let:loading let:error>
  <Navbar {logo} />

  <div class="main-container">
    <div>
      <a href="https://github.com/cassieopeanuts/holochan" target="_blank">
        <img src={logo} class="logo holochan" alt="holochan logo" />
      </a>
    </div>

    <h1>HoloChan</h1>

    <StatusCard {loading} {error} />

    {#if author}
      <ThreadsSection {author} imageHash={yourImageHash} />
      <PostsSection {author} threadHash={yourThreadHash} imageHash={yourImageHash} />
    {:else}
      <p>Loading user information...</p>
    {/if}
  </div>
</ClientProvider>

<style>
/* Main container and logo styles from App.svelte */
</style>
