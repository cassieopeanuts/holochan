<script context="module" lang="ts">
  // We can define slot props if we want to expose client, loading, error via <slot {client} ... />
  export interface ClientProviderSlotProps {
    client: AppWebsocket | null;
    loading: boolean;
    error: string | null;
  }
</script>

<script lang="ts">
  import { onMount, setContext } from "svelte";
  import { AppWebsocket } from "@holochain/client";
  import { clientContext, type ClientContextValue } from "../contexts";

  let client: AppWebsocket | null = null;
  let loading = false;
  let error: string | null = null;

  // Provide the typed context
  setContext<ClientContextValue>(clientContext, {
    getClient: () => client
  });

  onMount(async () => {
    loading = true;
    try {
      client = await AppWebsocket.connect(); // or pass a URL
    } catch (e) {
      error = (e as Error).message;
      console.error("Failed to connect to Holochain:", e);
    } finally {
      loading = false;
    }
  });
</script>

<slot {client} {loading} {error} />
