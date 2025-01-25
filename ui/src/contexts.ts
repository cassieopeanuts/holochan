// src/contexts.ts
import type { AppWebsocket } from "@holochain/client";

/** A symbol uniquely identifying our context key */
export const clientContext = Symbol("holochain-client");

/** The shape of the object you'll provide in setContext() */
export interface ClientContextValue {
  getClient: () => AppWebsocket | null;
}
