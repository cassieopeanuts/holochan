<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Thread } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalThreadHash!: ActionHash;

let currentThread: Thread = decode((currentRecord.entry as any).Present.entry) as Thread;
let title: string | undefined = currentThread.title;
let content: string | undefined = currentThread.content;
let timestamp: number | undefined = currentThread.timestamp;
let board: string | undefined = currentThread.board;

$: title, content, timestamp, board;
$: isThreadValid = true && title !== "" && content !== "" && true && board !== "";

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditThread element`);
  }
  if (!originalThreadHash) {
    throw new Error(`The originalThreadHash input is required for the EditThread element`);
  }
  client = await appClientContext.getClient();
});

async function updateThread() {
  const thread: Thread = {
    title: title!,
    content: content!,
    timestamp: timestamp!,
    board: board!,
    author: currentThread.author,
    image_hash: currentThread.image_hash,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "update_thread",
      payload: {
        original_thread_hash: originalThreadHash,
        previous_thread_hash: currentRecord.signed_action.hashed.hash,
        updated_thread: thread,
      },
    });

    dispatch("thread-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
  <div>
    <label for="Title">Title</label>
    <textarea name="Title" bind:value={title} required />
  </div>
  <div>
    <label for="Content">Content</label>
    <textarea name="Content" bind:value={content} required />
  </div>
  <div>
    <label for="Timestamp">Timestamp</label>
    <input
      name="Timestamp"
      type="datetime-local"
      value={new Date(timestamp / 1000 - (new Date(timestamp / 1000).getTimezoneOffset() * 60000)).toISOString().slice(0, 16)}
      on:input={(e) => timestamp = Math.floor(new Date(e.currentTarget.value).getTime() / 1000)}
      required
    />
  </div>
  <div>
    <label for="Board">Board</label>
    <input name="Board" bind:value={board} required />
  </div>

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isThreadValid} on:click={() => updateThread()}>
      Edit Thread
    </button>
  </div>
</section>
