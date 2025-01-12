<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditThread from "./EditThread.svelte";
import type { Thread } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let thread: Thread | undefined;

export let threadHash: ActionHash;

$: editing, error, loading, record, thread;

onMount(async () => {
  if (threadHash === undefined) {
    throw new Error(`The threadHash input is required for the ThreadDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchThread();
});

async function fetchThread() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "get_latest_thread",
      payload: threadHash,
    });
    if (record) {
      thread = decode((record.entry as any).Present.entry) as Thread;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteThread() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "delete_thread",
      payload: threadHash,
    });
    dispatch("thread-deleted", { threadHash: threadHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the thread: {error.message}</div>
{:else if editing}
  <EditThread
    originalThreadHash={threadHash}
    currentRecord={record}
    on:thread-updated={async () => {
      editing = false;
      await fetchThread();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <section>
    <div>
      <span><strong>Title:</strong></span>
      <span>{thread?.title}</span>
    </div>
    <div>
      <span><strong>Content:</strong></span>
      <span>{thread?.content}</span>
    </div>
    <div>
      <span><strong>Timestamp:</strong></span>
      <span>{new Date(thread?.timestamp / 1000).toLocaleString()}</span>
    </div>
    <div>
      <span><strong>Board:</strong></span>
      <span>{thread?.board}</span>
    </div>

    <div>
      <button
        on:click={() => {
          editing = true;
        }}
      >edit</button>
      <button on:click={() => deleteThread()}>delete</button>
    </div>
  </section>
{/if}
