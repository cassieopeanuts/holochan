<script lang="ts">
  import { onMount } from "svelte";
  import ClientProvider from "./components/ClientProvider.svelte";
  import HomePage from "./elements/Homepage.svelte";
  import type { ClientProviderSlotProps } from "./components/ClientProvider.svelte";
  import type { AppWebsocket, CellId, AgentPubKey } from "@holochain/client";
  import { encodeCellIdToBase64 } from "./lib/utils";

  let loading = false;
  let error: string | undefined = undefined;
  let threads = [];

  // Will be populated once we connect to Holochain and fetch appInfo
  let cellIdB64: string | null = null;
  let agentPubKey: AgentPubKey | null = null;

  /**
   * Handles creating a new thread
   */
  function handleCreateThread({ title, content, timestamp, board }) {
    threads = [
      ...threads,
      {
        hash: `${Date.now()}`, // Replace with actual hash from backend
        title,
        content,
        timestamp,
        board,
      },
    ];
  }

  /**
   * Called from ClientProvider to set the client,
   * but the actual client is also made available via context
   */
  async function onClientReady(client: ClientProviderSlotProps["client"]) {
    try {
      loading = true;

      // Fetch app info and initialize
      const installed_app_id = "imageboard";
      const appInfo = await client.appInfo({ installed_app_id });

      const cellInfo = appInfo.cell_info["imageboard"]?.[0];
      if (!cellInfo) {
        throw new Error(
          `No cell_info found for role_name "imageboard" in installed_app_id="${installed_app_id}"`
        );
      }

      let cellId: CellId;
      if ("provisioned" in cellInfo) {
        cellId = cellInfo.provisioned.cell_id;
      } else if ("cloned" in cellInfo) {
        cellId = cellInfo.cloned.cell_id;
      } else {
        throw new Error("Unable to find a valid cellId in cellInfo.");
      }

      cellIdB64 = encodeCellIdToBase64(cellId);

      // Fetch agent public key
      agentPubKey = await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "get_agent_pub_key",
        payload: null,
      });

      // Fetch threads
      const links = await client.callZome({
        cap_secret: null,
        role_name: "imageboard",
        zome_name: "posts",
        fn_name: "get_all_threads",
        payload: null,
      });

      threads = links.map((link) => ({
        hash: link.target,
        title: `Thread ${link.target.slice(0, 6)}`, // Replace with actual data
        content: `Content for ${link.target.slice(0, 6)}`, // Replace with actual data
        timestamp: Date.now(), // Replace with actual data
      }));
    } catch (e) {
      console.error("Error initializing Holochain client:", e);
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<ClientProvider let:client let:loading let:error on:connected={(e) => onClientReady(e.detail.client)}>
  <main>
    <HomePage {threads} onCreateThread={handleCreateThread} />
  </main>
</ClientProvider>
