<script lang="ts">

import { writable, get } from 'svelte/store';
import { FileStorageClient } from '@holochain-open-dev/file-storage';
import { FileStatus } from '$lib/types';
import { decodeCellIdFromBase64 } from '$lib/utils';
import { decodeHashFromBase64 } from '@holochain/client';
import pRetry from 'p-retry';

export function createGenericKeyValueStore() {
  const store = writable({});

  return {
    subscribe: store.subscribe,
    update: (callback) => {
      store.update((current) => callback(current));
    },
  };
}

export function createFileStore(client) {
  const data = createGenericKeyValueStore();

  async function upload(cellIdB64, file) {
    const fileStorageClient = new FileStorageClient(
      client.client,
      'UNUSED ROLE NAME', // Not used but required
      'file_storage',
      decodeCellIdFromBase64(cellIdB64)
    );
    return fileStorageClient.uploadFile(file);
  }

  async function download(cellIdB64, entryHashB64) {
    const currentData = get(data);
    if (
      currentData[entryHashB64]?.file &&
      currentData[entryHashB64].status === FileStatus.Loaded
    ) {
      return;
    }

    data.update((c) => ({
      ...c,
      [entryHashB64]: {
        file: undefined,
        status: FileStatus.Loading,
      },
    }));

    const fileStorageClient = new FileStorageClient(
      client.client,
      'UNUSED ROLE NAME',
      'file_storage',
      decodeCellIdFromBase64(cellIdB64)
    );

    try {
      const file = await pRetry(
        () => fileStorageClient.downloadFile(decodeHashFromBase64(entryHashB64)),
        {
          retries: 10,
          minTimeout: 1000,
          maxTimeout: 10000,
          factor: 2,
          onFailedAttempt: (e) => {
            console.error(
              `Failed attempt ${e.attemptNumber} to download file from hash ${entryHashB64}`,
              e
            );
            if (e.retriesLeft === 0) {
              throw new Error('Failed to download file after 10 retries, giving up.');
            }
          },
        }
      );

      data.update((c) => ({
        ...c,
        [entryHashB64]: {
          file,
          status: FileStatus.Loaded,
        },
      }));
    } catch (e) {
      data.update((c) => ({
        ...c,
        [entryHashB64]: {
          file: undefined,
          status: FileStatus.Error,
        },
      }));
    }
  }

  return {
    subscribe: data.subscribe,
    update: data.update,
    upload,
    download,
  };
}

export function deriveCellFileStore(fileStore, key) {
  return {
    ...fileStore,
    upload: (file) => fileStore.upload(key, file),
    download: (entryHashB64) => fileStore.download(key, entryHashB64),
  };
}

</script>