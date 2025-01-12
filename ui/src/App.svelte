<script lang="ts">
  import type { ActionHash, AppClient, HolochainError, HoloHash } from "@holochain/client";
  import { AppWebsocket } from "@holochain/client";
  import { onMount, setContext } from "svelte";
  
  import { AdminWebsocket } from "@holochain/client";

  import logo from "./assets/holo_chan.png";
  import { type ClientContext, clientContext } from "./contexts";
  
  import fs from "fs";

  // Import your scaffolded components
  import AllPosts from './imageboard/posts/AllPosts.svelte';
  import PostsByThread from './imageboard/posts/PostsByThread.svelte';
  import AllThreads from "./imageboard/posts/AllThreads.svelte";
  import CreatePost from "./imageboard/posts/CreatePost.svelte";
  import CreateThread from "./imageboard/posts/CreateThread.svelte";
  import EditPost from "./imageboard/posts/EditPost.svelte";
  import EditThread from "./imageboard/posts/EditThread.svelte";
  import PostDetail from "./imageboard/posts/PostDetail.svelte";
  import PostsForCreator from "./imageboard/posts/PostsForCreator.svelte";
  import PostsForPost from "./imageboard/posts/PostsForPost.svelte";
  import PostsForThread from "./imageboard/posts/PostsForThread.svelte";
  import ThreadDetail from "./imageboard/posts/ThreadDetail.svelte";
  import ThreadsForThread from "./imageboard/posts/ThreadsForThread.svelte";

  let client: AppClient | undefined;
  let error: HolochainError | undefined;
  let loading = false;
  let author: HoloHash | undefined; // Define author variable
  let yourImageHash: HoloHash | undefined;
  let yourThreadHash: HoloHash | undefined;

  const appClientContext = {
  getClient: async () => {
    if (!client) {
      try {
        client = await AppWebsocket.connect();
      } catch (e) {
        console.error("Failed to connect to AppWebsocket:", e);
        throw e; // Rethrow to handle in onMount
      }
    }
    return client;
  },
};

  async function getDnaHashAndCellId() {
  const adminClient = await AdminWebsocket.connect({
    url: new URL("ws://localhost:8888"), // Create a URL object
  });
  
  const cells = await adminClient.listCellIds();
  
  // Assuming you have only one cell, or loop through if multiple
  const cell = cells[0]; // Get the first cell
  return {
    dnaHash: cell[0], // The DNA hash
    cellId: cell[1], // The CellId which includes AgentPubKey
  };
}

async function getCurrentUserKey(): Promise<HoloHash> {
  if (!client) {
    throw new Error("Client is not initialized");
  }

  const { dnaHash, cellId } = await getDnaHashAndCellId(); // Retrieve DNA hash and Cell ID
  
  try {
    const response = await client.callZome({
      cell_id: [dnaHash, cellId], // Use retrieved values here
      zome_name: "posts", // Replace with your actual zome name
      fn_name: "get_agent_pub_key", // The name of the zome function
      payload: null,
    });

    return response as HoloHash; // Ensure this matches the expected type
  } catch (error) {
    console.error("Error fetching user key:", error);
    throw error; // Rethrow or handle as needed
  }
}

onMount(async () => {
  try {
    loading = true;
    client = await appClientContext.getClient();
    author = await getCurrentUserKey();
  } catch (e) {
    console.error("Error during onMount:", e); // Log detailed error
    error = e as HolochainError;
  } finally {
    loading = false;
  }
});
 
  setContext<ClientContext>(clientContext, appClientContext);
</script>

<main>
  <div>
    <a href="https://github.com/cassieopeanuts/holochan" target="_blank">
      <img src={logo} class="logo holochan" alt="holochan logo" />
    </a>
  </div>
  
  <h1>HoloChan</h1>
  
  <div>
    <div class="card">
      {#if loading}
        <p>Connecting...</p>
      {:else if error}
        <p>{error.message}</p>
      {:else}
        <p>Client is connected.</p>
      {/if}
    </div>
    
    {#if author}
    <CreateThread {author} imageHash={yourImageHash} />
    <AllThreads {author} />
    
    <!-- Assuming you have logic to determine imageHash and threadHash -->
    <CreatePost {author} imageHash={yourImageHash} threadHash={yourThreadHash} />
    <AllPosts {author} />
    
    <PostsByThread {author} />
  {:else}
    <p>Loading user information...</p>
  {/if}
  

    <p class="read-the-docs">Click on the Holochain logo to learn more</p>
  </div>
</main>

<style>
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

.card {
  padding: 2em;
}

.read-the-docs {
  color: #888;
}
</style>