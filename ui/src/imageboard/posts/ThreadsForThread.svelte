<script lang="ts">
import type {
  ActionHash,
  AgentPubKey,
  AppClient,
  EntryHash,
  HolochainError,
  Link,
  NewEntryAction,
  Record,
  SignalType,
} from "@holochain/client";
import { SignalType } from "@holochain/client";
import { getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import ThreadDetail from "./ThreadDetail.svelte";
import type { PostsSignal, Thread } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let hashes: Array<ActionHash> | undefined = [];
let loading: boolean;
let error: any = undefined;

export let threadHash: EntryHash;

$: hashes, loading, error;

onMount(async () => {
  if (threadHash === undefined) {
    throw new Error(`The threadHash input is required for the ThreadsForThread element`);
  }
  client = await appClientContext.getClient();
  await fetchThreads();

  client.on("signal", async signal => {
    if (!(SignalType.App in signal)) return;
    if (signal.App.zome_name !== "posts") return;
    const payload = signal.App.payload as PostsSignal;
    if (!(payload.type === "EntryCreated" && payload.app_entry.type === "Thread")) return;
    await fetchThreads();
  });
});

async function fetchThreads() {
  loading = true;
  try {
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "get_threads_for_thread",
      payload: threadHash,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching threads: ${error.message}.</div>
{:else if hashes.length === 0}
  <div class="alert">No threads found for this thread.</div>
{:else}
  <div>
    {#each hashes as hash}
      <ThreadDetail threadHash={hash} on:thread-deleted={fetchThreads} />
    {/each}
  </div>
{/if}
