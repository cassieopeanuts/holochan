<script lang="ts">
    import type { HoloHash } from "@holochain/client";
    import CreateThread from "../imageboard/posts/CreateThread.svelte";
    import AllThreads from "../imageboard/posts/AllThreads.svelte";
  
    export let author: HoloHash;
    export let imageHash: HoloHash;
  </script>
  
  <section class="threads-section">
    <h2>Your Threads</h2>
    <CreateThread {author} {imageHash} />
    <AllThreads />
  </section>
  
  <style>
  /* Styles from App.svelte's .threads-section */
  </style>
  