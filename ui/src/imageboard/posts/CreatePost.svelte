<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Post } from "./types";
import CreatePostForm from "../../elements/CreatePostForm.svelte";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let content: string = "";
let timestamp: number = Date.now();

export let author!: AgentPubKey;
export let threadHash!: EntryHash;
export let imageHash: EntryHash | undefined;

$: content, author, timestamp, threadHash, imageHash;
$: isPostValid = true && content !== "" && true;

onMount(async () => {
  if (author === undefined) {
    throw new Error(`The author input is required for the CreatePost element`);
  }
  if (threadHash === undefined) {
    throw new Error(`The threadHash input is required for the CreatePost element`);
  }
  if (imageHash === undefined) {
    console.log("No imageHash provided for this post");
  }

  client = await appClientContext.getClient();
});

async function createPost() {
  const postEntry: Post = {
    content: content!,
    author: author!,
    timestamp: timestamp!,
    thread_hash: threadHash!,
    image_hash: imageHash,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "create_post",
      payload: postEntry,
    });
    dispatch("post-created", { postHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<CreatePostForm
  bind:content
  bind:timestamp
  {isPostValid}
  onCreatePost={createPost}
/>