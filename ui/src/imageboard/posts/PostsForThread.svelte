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
} from "@holochain/client";
import { SignalType } from "@holochain/client";
import { getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import PostDetail from "./PostDetail.svelte";
import type { PostsSignal } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let hashes: Array<ActionHash> | undefined;
let loading = false;
let error: any = undefined;

export let threadHash: ActionHash;

$: hashes, loading, error;

onMount(async () => {
  if (!threadHash) {
    throw new Error(`The threadHash input is required for the PostsForThread element`);
  }
  client = await appClientContext.getClient();
  try {
    loading = true;
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "get_posts_for_thread",
      payload: threadHash,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }

  client.on("signal", signal => {
    if (!(SignalType.App in signal)) return;
    if (signal.App.zome_name !== "posts") return;
    const payload = signal.App.payload as PostsSignal;
    if (payload.type !== "LinkCreated") return;
    if (payload.link_type !== "ThreadToPosts") return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching posts: {error.message}.</div>
{:else if hashes.length === 0}
  <div class="alert">No posts found for this thread.</div>
{:else}
  <div>
    {#each hashes as hash}
      <PostDetail postHash={hash} />
    {/each}
  </div>
{/if}
