import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  hashFrom32AndType,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function sampleThread(cell: CallableCell, partialThread = {}) {
  return {
    ...{
      title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      author: cell.cell_id[1],
      timestamp: 1674053334548000,
      board: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      image_hash: null,
    },
    ...partialThread,
  };
}

export async function createThread(cell: CallableCell, thread = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "posts",
    fn_name: "create_thread",
    payload: thread || await sampleThread(cell),
  });
}

export async function samplePost(cell: CallableCell, partialPost = {}) {
  return {
    ...{
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      author: (await fakeAgentPubKey()),
      timestamp: 1674053334548000,
      thread_hash: (await fakeEntryHash()),
      image_hash: null,
    },
    ...partialPost,
  };
}

export async function createPost(cell: CallableCell, post = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "posts",
    fn_name: "create_post",
    payload: post || await samplePost(cell),
  });
}

export async function sampleBoard(cell: CallableCell, partialBoard = {}) {
  return {
    ...{
      name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      created_at: 1674053334548000,
      creator: cell.cell_id[1],
      rules: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      category: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    },
    ...partialBoard,
  };
}

export async function createBoard(cell: CallableCell, board = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "posts",
    fn_name: "create_board",
    payload: board || await sampleBoard(cell),
  });
}
