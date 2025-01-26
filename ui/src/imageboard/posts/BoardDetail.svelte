<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditBoard from "./EditBoard.svelte";
import type { Board } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let board: Board | undefined;

export let boardHash: ActionHash;

$: editing, error, loading, record, board;

onMount(async () => {
  if (boardHash === undefined) {
    throw new Error(`The boardHash input is required for the BoardDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchBoard();
});

async function fetchBoard() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "get_latest_board",
      payload: boardHash,
    });
    if (record) {
      board = decode((record.entry as any).Present.entry) as Board;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteBoard() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "imageboard",
      zome_name: "posts",
      fn_name: "delete_board",
      payload: boardHash,
    });
    dispatch("board-deleted", { boardHash: boardHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the board: {error.message}</div>
{:else if editing}
  <EditBoard
    originalBoardHash={boardHash}
    currentRecord={record}
    on:board-updated={async () => {
      editing = false;
      await fetchBoard();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <section>
    <div>
      <span><strong>Name:</strong></span>
      <span>{board?.name}</span>
    </div>
    <div>
      <span><strong>Description:</strong></span>
      <span>{board?.description}</span>
    </div>
    <div>
      <span><strong>Rules:</strong></span>
      <span>{board?.rules}</span>
    </div>
    <div>
      <span><strong>Category:</strong></span>
      <span>{board?.category}</span>
    </div>

    <div>
      <button
        on:click={() => {
          editing = true;
        }}
      >edit</button>
      <button on:click={() => deleteBoard()}>delete</button>
    </div>
  </section>
{/if}
