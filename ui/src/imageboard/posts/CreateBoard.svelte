<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Board } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let name: string = "";
let description: string = "";
let rules: string = "";
let category: string = "";

export let createdAt!: number;
export let creator!: AgentPubKey;

$: name, description, createdAt, creator, rules, category;
$: isBoardValid = true && name !== "" && description !== "" && rules !== "" && category !== "";

onMount(async () => {
  if (createdAt === undefined) {
    throw new Error(`The createdAt input is required for the CreateBoard element`);
  }
  if (creator === undefined) {
    throw new Error(`The creator input is required for the CreateBoard element`);
  }
  client = await appClientContext.getClient();
});

async function createBoard() {
  const boardEntry: Board = {
    name: name!,
    description: description!,
    created_at: createdAt!,
    creator: creator!,
    rules: rules!,
    category: category!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "create_board",
      payload: boardEntry,
    });
    dispatch("board-created", { boardHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Board</h3>

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

  <button disabled={!isBoardValid} on:click={() => createBoard()}>
    Create Board
  </button>
</div>
