<!-- projects/holochan/ui/src/App.svelte -->

<script lang="ts">
  import { encodeCellIdToBase64 } from "./lib/utils";
  import ClientProvider from "./components/ClientProvider.svelte";
  import HomePage from "./elements/Homepage.svelte";
  import type { HoloHash } from "@holochain/client";

  let loading = false;
  let error: string | null = null;
  let threads: Array<{
    title: string;
    content: string;
    boardHash: string;
    timestamp: number;
    hash: HoloHash;
  }> = [];
  let boards = [
    { name: "Announcements", description: "Official updates.", hash: "announcements" },
    { name: "Random", description: "Anything goes.", hash: "random" },
  ];
  let client: any = null;
  let cellIdB64: string | null = null;

  function onConnected(event: CustomEvent<{ client: any; dnaHash: HoloHash; agentPubKey: HoloHash }>) {
    const { client: connectedClient, dnaHash, agentPubKey } = event.detail;
    console.log("App.svelte: Client connected:", connectedClient);
    console.log("App.svelte: DNA hash:", dnaHash);
    console.log("App.svelte: Agent PubKey:", agentPubKey);

    client = connectedClient;

    cellIdB64 = encodeCellIdToBase64([agentPubKey, dnaHash]);
    console.log("App.svelte: cellIdB64:", cellIdB64);
  }

  function handleCreateThread({ title, content, boardHash, fileHash }: { title: string; content: string; boardHash: string; fileHash: HoloHash }) {
    threads = [
      ...threads,
      { title, content, boardHash, timestamp: Date.now(), hash: fileHash },
    ];
  }
</script>

<ClientProvider on:connected={onConnected}>
  <main>
    {#if client && cellIdB64}
      <HomePage
        {threads}
        {boards}
        {client}
        {cellIdB64}
        onCreateThread={handleCreateThread}
        onFileUploaded={() => console.log("Mock file uploaded")}
      />
    {:else}
      <p>Loading client and cellIdB64...</p>
    {/if}
  </main>
</ClientProvider>
