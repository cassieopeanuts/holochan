<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Board } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalBoardHash!: ActionHash;

let currentBoard: Board = decode((currentRecord.entry as any).Present.entry) as Board;
let name: string | undefined = currentBoard.name;
let description: string | undefined = currentBoard.description;
let rules: string | undefined = currentBoard.rules;
let category: string | undefined = currentBoard.category;

$: name, description, rules, category;
$: isBoardValid = true && name !== "" && description !== "" && rules !== "" && category !== "";

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditBoard element`);
  }
  if (!originalBoardHash) {
    throw new Error(`The originalBoardHash input is required for the EditBoard element`);
  }
  client = await appClientContext.getClient();
});

async function updateBoard() {
  const board: Board = {
    name: name!,
    description: description!,
    rules: rules!,
    category: category!,
    created_at: currentBoard.created_at,
    creator: currentBoard.creator,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "update_board",
      payload: {
        original_board_hash: originalBoardHash,
        previous_board_hash: currentRecord.signed_action.hashed.hash,
        updated_board: board,
      },
    });

    dispatch("board-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
  <div>
    <label for="Name">Name</label>
    <input name="Name" bind:value={name} required />
  </div>
  <div>
    <label for="Description">Description</label>
    <textarea name="Description" bind:value={description} required />
  </div>
  <div>
    <label for="Rules">Rules</label>
    <textarea name="Rules" bind:value={rules} required />
  </div>
  <div>
    <label for="Category">Category</label>
    <input name="Category" bind:value={category} required />
  </div>

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isBoardValid} on:click={() => updateBoard()}>
      Edit Board
    </button>
  </div>
</section>
