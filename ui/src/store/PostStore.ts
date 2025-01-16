import { writable } from "svelte/store";
import type { ActionHash, Link, AppClient, Signal } from "@holochain/client";

const posts = writable<ActionHash[]>([]);
const loading = writable(false);
const error = writable<string | null>(null);

let holochainClient: AppClient | null = null;

export async function initializeClient(client: AppClient) {
  holochainClient = client;
}

export async function fetchPostsForThread(threadHash: ActionHash) {
  if (!holochainClient) {
    throw new Error("Holochain client is not initialized.");
  }

  loading.set(true);
  error.set(null);

  try {
    const links: Array<Link> = await holochainClient.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "get_posts_for_thread",
      payload: threadHash,
    });

    const postHashes = links.map(link => link.target);
    posts.set(postHashes);
  } catch (err) {
    error.set(err.message);
  } finally {
    loading.set(false);
  }
}

export function listenForPostSignals() {
  if (!holochainClient) {
    throw new Error("Holochain client is not initialized.");
  }

  holochainClient.on("signal", (signal: Signal) => {
    if (!("App" in signal)) return;

    const { App } = signal;
    if (App.zome_name !== "posts") return;

    const payload = App.payload as {
      type: string;
      app_entry: { type: string };
      action: { hashed: { hash: ActionHash } };
    };
    if (payload.type === "EntryCreated" && payload.app_entry.type === "Post") {
      posts.update(currentPosts => [...currentPosts, payload.action.hashed.hash]);
    }
  });
}

export { posts, loading, error };
