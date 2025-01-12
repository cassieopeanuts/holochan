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
