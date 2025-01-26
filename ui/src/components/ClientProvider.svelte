<!-- projects/holochan/ui/src/components/ClientProvider.svelte -->

<script lang="ts">
  import { onMount, setContext } from "svelte";
  import { AppWebsocket, encodeHashToBase64 } from "@holochain/client";
  import { clientContext, type ClientContext } from "../contexts";
  import { createEventDispatcher } from "svelte";
  import { type HoloHash } from "@holochain/client";
  import type { ProvisionedCell, AppInfo, CellInfo } from "@holochain/client";
  import { CellType } from "@holochain/client";

  const dispatch = createEventDispatcher();

  let client: AppWebsocket | null = null;
  let dnaHash: HoloHash = new Uint8Array(); // Initialize as HoloHash
  let loading = false;
  let error: string | null = null;

  console.log("Initializing ClientProvider with DNA hash:", dnaHash);

  setContext<ClientContext>(clientContext, {
    getClient: async () => {
      if (!client) {
        console.log("Connecting to Holochain...");
        client = await AppWebsocket.connect();
        console.log("Client connected:", client);
      }
      return client;
    },
    getDnaHash: () => dnaHash,
  });


  onMount(async () => {
    loading = true;
    try {
      client = await AppWebsocket.connect();
      const appInfo = await client.appInfo();

      // Get first role ID from the cell_info map
      const roleIds = Object.keys(appInfo.cell_info);
      if (roleIds.length === 0) throw new Error("No roles found in app info");
      
      // Get first cell info for first role
      const roleCells = appInfo.cell_info[roleIds[0]];
      if (!roleCells || roleCells.length === 0) throw new Error("No cells found for first role");
      
      // Type-safe cell info handling
      const cellInfo = roleCells[0];
      if (CellType.Provisioned in cellInfo) {
        const provisionedCell = cellInfo[CellType.Provisioned] as ProvisionedCell;
        const [dnaHash, agentPubKey] = provisionedCell.cell_id;
        
        console.log("DNA Hash:", encodeHashToBase64(dnaHash));
        console.log("Agent Public Key:", encodeHashToBase64(agentPubKey));
        
        dispatch("connected", { client, dnaHash, agentPubKey });
      } else {
        throw new Error("First cell is not a provisioned cell");
      }
    } catch (e) {
      error = (e as Error).message;
      console.error("Error connecting to Holochain:", e);
    } finally {
      loading = false;
    }
  });
</script>

<slot {client} {loading} {error} />
