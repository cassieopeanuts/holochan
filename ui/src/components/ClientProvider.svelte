<!-- src/components/ClientProvider.svelte -->
<script lang="ts">
    import { onMount, setContext } from "svelte";
    import { AppWebsocket } from "@holochain/client";
    import { clientContext } from "../contexts";
  
    let client;
    let loading = false;
    let error;
  
    setContext(clientContext, {
      getClient: () => client,
    });
  
    onMount(async () => {
      try {
        loading = true;
        client = await AppWebsocket.connect();
      } catch (e) {
        error = e.message;
      } finally {
        loading = false;
      }
    });
  </script>
  
  <slot {loading} {error} />
  