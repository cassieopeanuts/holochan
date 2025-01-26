// projects/holochan/ui/src/store/FileStore.ts

import { FileStorageClient } from "@holochain-open-dev/file-storage";
import { type HoloHash } from "@holochain/client";
import type { CellId } from "@holochain/client";

export function createFileStore(client: any, cellId: CellId) {
  const fileStorageClient = new FileStorageClient(client, "imageboard", "file_storage", cellId);

  return {
    upload: async (file: File): Promise<HoloHash> => {
      return await fileStorageClient.uploadFile(file);
    },
    download: async (entryHash: HoloHash): Promise<Blob> => {
      return await fileStorageClient.downloadFile(entryHash);
    },
  };
}
