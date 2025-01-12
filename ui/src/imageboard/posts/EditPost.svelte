<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Post } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalPostHash!: ActionHash;

let currentPost: Post = decode((currentRecord.entry as any).Present.entry) as Post;
let content: string | undefined = currentPost.content;
let timestamp: number | undefined = currentPost.timestamp;

$: content, timestamp;
$: isPostValid = true && content !== "" && true;

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditPost element`);
  }
  if (!originalPostHash) {
    throw new Error(`The originalPostHash input is required for the EditPost element`);
  }
  client = await appClientContext.getClient();
});

async function updatePost() {
  const post: Post = {
    content: content!,
    timestamp: timestamp!,
    author: currentPost.author,
    thread_hash: currentPost.thread_hash,
    image_hash: currentPost.image_hash,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "update_post",
      payload: {
        original_post_hash: originalPostHash,
        previous_post_hash: currentRecord.signed_action.hashed.hash,
        updated_post: post,
      },
    });

    dispatch("post-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
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
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isPostValid} on:click={() => updatePost()}>
      Edit Post
    </button>
  </div>
</section>
