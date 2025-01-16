<script lang="ts">
  import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import { type ClientContext, clientContext } from "../../contexts";
  import type { Thread } from "./types";
  import CreateThreadForm from "../../elements/CreateThreadForm.svelte";


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
  
  <CreateThreadForm
  bind:title
  bind:content
  bind:timestamp
  bind:board
  {isThreadValid}
  onCreateThread={createThread}
/>