<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Thread } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let title: string = "";
let content: string = "";
let timestamp: number = Date.now();
let board: string = "";

export let author!: AgentPubKey;
export let imageHash: EntryHash | undefined;

$: title, content, author, timestamp, board, imageHash;
$: isThreadValid = true && title !== "" && content !== "" && true && board !== "";

onMount(async () => {
  if (author === undefined) {
    throw new Error(`The author input is required for the CreateThread element`);
  }
  client = await appClientContext.getClient();
});

async function createThread() {
  const threadEntry: Thread = {
    title: title!,
    content: content!,
    author: author!,
    timestamp: timestamp!,
    board: board!,
    image_hash: imageHash,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "create_thread",
      payload: threadEntry,
    });
    dispatch("thread-created", { threadHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Thread</h3>

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

  <button disabled={!isThreadValid} on:click={() => createThread()}>
    Create Thread
  </button>
</div>
