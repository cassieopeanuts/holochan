import { writable } from "svelte/store";
import type { ActionHash, AppClient, Link, Signal, AgentPubKey } from "@holochain/client";

// Define store types
export interface ThreadStore {
  threads: ReturnType<typeof writable<ActionHash[]>>;
  loading: ReturnType<typeof writable<boolean>>;
  error: ReturnType<typeof writable<string | null>>;
  fetchThreads: () => Promise<void>;
  listenForSignals: () => void;
}

// Create and export the thread store
export function createThreadStore(client: AppClient) {
  const threads = writable<ActionHash[]>([]);
  const loading = writable<boolean>(false);
  const error = writable<string | null>(null);

  async function fetchThreads() {
    loading.set(true);
    error.set(null);
    try {
      const links: Link[] = await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "get_all_threads",
        payload: null,
      });
      threads.set(links.map(link => link.target as ActionHash));
    } catch (err) {
      error.set((err as Error).message);
    } finally {
      loading.set(false);
    }
  }

  function listenForSignals() {
    client.on("signal", (signal: Signal) => {
      if (!("App" in signal)) return;

      const { App } = signal;
      if (App.zome_name !== "posts") return;

      const payload = App.payload as {
        type: string;
        app_entry: { type: string };
        action: { hashed: { hash: ActionHash } };
      };

      if (payload.type === "EntryCreated" && payload.app_entry.type === "Thread") {
        threads.update(currentThreads => [...currentThreads, payload.action.hashed.hash]);
      }
    });
  }


  async function createThread(
    author: AgentPubKey,
    title: string,
    content: string,
    timestamp: number,
    board: string,
    imageHash?: Uint8Array // or HoloHash
  ) {
    try {
      // Construct payload according to your `create_thread` zome function
      const threadInput = {
        title,
        content,
        author,
        timestamp,
        board,
        image_hash: imageHash,
      };
      await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "create_thread",
        payload: threadInput,
      });

      // Refresh threads after creation
      fetchThreads();
    } catch (e) {
      error.set((e as Error).message);
    }
  }

  async function deleteThread(threadHash: ActionHash) {
    try {
      await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "delete_thread",
        payload: threadHash,
      });
      fetchThreads();
    } catch (e) {
      error.set((e as Error).message);
    }
  }

  return {
    threads,
    loading,
    error,
    fetchThreads,
    listenForSignals,
    createThread,
    deleteThread,
  };
}