import type {
  ActionHash,
  AgentPubKey,
  Create,
  CreateLink,
  Delete,
  DeleteLink,
  DnaHash,
  EntryHash,
  ExternalHash,
  Record,
  SignedActionHashed,
  Update,
} from "@holochain/client";

export type PostsSignal = {
  type: "EntryCreated";
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: "EntryUpdated";
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: "EntryDeleted";
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: "LinkCreated";
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: "LinkDeleted";
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

/* dprint-ignore-start */
export type EntryTypes =
 | ({ type: 'Post'; } & Post)
 | ({  type: 'Thread'; } & Thread);
/* dprint-ignore-end */

export interface Thread {
  title: string;
  content: string;
  author: AgentPubKey;
  timestamp: number;
  board: string;
  image_hash: EntryHash | undefined;
}

export interface Post {
  content: string;
  author: AgentPubKey;
  timestamp: number;
  thread_hash: EntryHash;
  image_hash: EntryHash | undefined;
}
