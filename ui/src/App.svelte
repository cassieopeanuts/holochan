<script lang="ts">
  import type {
    HoloHash,
    HolochainError,
    AppClient
  } from "@holochain/client";
  import { AppWebsocket } from "@holochain/client";
  import { onMount, setContext } from "svelte";

  import logo from "./assets/holo_chan.png";
  import { type ClientContext, clientContext } from "./contexts";

  import AllPosts from './imageboard/posts/AllPosts.svelte';
  import PostsByThread from './imageboard/posts/PostsByThread.svelte';
  import AllThreads from "./imageboard/posts/AllThreads.svelte";
  import CreatePost from "./imageboard/posts/CreatePost.svelte";
  import CreateThread from "./imageboard/posts/CreateThread.svelte";

  let client: AppClient | undefined;
  let error: HolochainError | undefined;
  let loading = false;
  let author: HoloHash | undefined; // The current user's agent pub key
  let yourImageHash: HoloHash | undefined; // Placeholder for imageHash
  let yourThreadHash: HoloHash | undefined; // Placeholder for threadHash

  // Provide a context object so child components can retrieve the Holochain client
  const appClientContext = {
    getClient: () => client,
  };

  async function getCurrentUserKey(): Promise<HoloHash> {
    if (!client) {
      throw new Error("Client is not initialized");
    }
    try {
      const response = await client.callZome({
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "get_agent_pub_key",
        payload: null,
      });
      return response as HoloHash;
    } catch (error) {
      console.error("Error fetching user key:", error);
      throw error;
    }
  }

  onMount(async () => {
    try {
      loading = true;

      client = await AppWebsocket.connect();

      author = await getCurrentUserKey();

      // Assign placeholders for demonstration
      yourImageHash = "example-image-hash" as unknown as HoloHash;
      yourThreadHash = "example-thread-hash" as unknown as HoloHash;
    } catch (e) {
      console.error("Error during onMount:", e);
      error = e as HolochainError;
    } finally {
      loading = false;
    }
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main>
  <!-- Navigation Bar -->
  <nav class="navbar">
    <div class="container">
      <img src={logo} alt="HoloChan Logo" class="logo small" />
      <ul class="nav-links">
        <li><a href="#home">Home</a></li>
        <li><a href="#threads">Threads</a></li>
        <li><a href="#profile">Profile</a></li>
        <li><a href="#create">Create</a></li>
      </ul>
    </div>
  </nav>

  <div class="main-container">
    <div>
      <a href="https://github.com/cassieopeanuts/holochan" target="_blank">
        <img src={logo} class="logo holochan" alt="holochan logo" />
      </a>
    </div>

    <h1>HoloChan</h1>

    <div class="status-card">
      {#if loading}
        <p>Connecting...</p>
      {:else if error}
        <p class="error">{error.message}</p>
      {:else}
        <p>Client is connected.</p>
      {/if}
    </div>

    {#if author}
      <section class="threads-section">
        <h2>Your Threads</h2>
        <CreateThread author={author} imageHash={yourImageHash} />
        <AllThreads author={author} />
      </section>

      <section class="posts-section">
        <h2>Your Posts</h2>
        <CreatePost author={author} threadHash={yourThreadHash} imageHash={yourImageHash} />
        <AllPosts author={author} />
      </section>

      <section class="threads-detail">
        <h2>Posts by Thread</h2>
        <PostsByThread author={author} />
      </section>
    {:else}
      <p>Loading user information...</p>
    {/if}
  </div>
</main>

<style>
/* Navigation Bar */
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background-color: #222;
  color: #fff;
}
.nav-links {
  display: flex;
  gap: 1rem;
  list-style: none;
}
.nav-links a {
  color: #61dafb;
  text-decoration: none;
}
.nav-links a:hover {
  text-decoration: underline;
}
.logo.small {
  height: 3em;
}

/* Main Container */
.main-container {
  padding: 2rem;
}

/* Sections */
.threads-section,
.posts-section,
.threads-detail {
  margin-bottom: 2rem;
}

h2 {
  font-size: 1.5rem;
  color: #444;
  margin-bottom: 1rem;
}

/* Cards */
.status-card {
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  background-color: #f9f9f9;
  margin-bottom: 2rem;
}
.error {
  color: #e74c3c;
  font-weight: bold;
}

/* Logo */
.logo {
  height: 15em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.holochan:hover {
  filter: drop-shadow(0 0 2em #61dafbaa);
}

/* Footer */
.read-the-docs {
  color: #888;
  margin-top: 2rem;
  text-align: center;
}

/* Responsive Design */
@media (max-width: 768px) {
  .navbar {
    flex-direction: column;
    align-items: flex-start;
  }
  .nav-links {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>
