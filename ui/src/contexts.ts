// projects/holochan/ui/src/contexts.ts

import type { AppClient, HoloHash } from "@holochain/client";

export const clientContext = "AppClient";

export type ClientContext = {
  getClient: () => Promise<AppClient>;
  getDnaHash: () => HoloHash;
};
