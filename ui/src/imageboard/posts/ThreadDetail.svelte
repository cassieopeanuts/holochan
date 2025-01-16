<script lang="ts">
  import { onMount, getContext } from "svelte";
  import { decode } from "@msgpack/msgpack";
  import { createEventDispatcher } from "svelte";
  import type { AppClient, ActionHash, Record } from "@holochain/client";
  import { type ClientContext, clientContext } from "../../contexts";
  import EditThread from "./EditThread.svelte";

  export let threadHash: ActionHash;

  const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

  let record: Record | undefined;
  let thread: any;
  let editing = false;
  let loading = false;
  let error: string | null = null;

  async function fetchThread() {
    try {
      loading = true;
      record = await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "get_latest_thread",
        payload: threadHash,
      });
      thread = decode((record.entry as any).Present.entry);
    } catch (e) {
      error = e.message;
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
      dispatch("thread-deleted", { threadHash });
    } catch (e) {
      alert(e.message);
    }
  }

  onMount(async () => {
    client = await appClientContext.getClient(); 
    await fetchThread();
  });
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error loading thread: {error}</div>
{:else if editing}
  <EditThread
    originalThreadHash={threadHash}
    currentRecord={record}
    on:thread-updated={async () => {
      editing = false;
      await fetchThread(); 
    }}
    on:edit-canceled={() => (editing = false)}
  />
{:else}
  <div>
    <h3>{thread?.title}</h3>
    <p>Thread: {threadHash}</p> 
    <button on:click={() => (editing = true)}>Edit</button>
    <button on:click={() => deleteThread()}>Delete</button>
  </div>
{/if}
