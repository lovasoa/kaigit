<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let id: string;
  export let path: string;
  export let observer: IntersectionObserver | null;
  let div: HTMLDivElement | null = null;
  export let visible = false;
  $: if (observer && div) observer.observe(div);
  let object: Promise<GitObject> = new Promise(() => {});
  let t: NodeJS.Timeout;
  $: if (visible)
    t = setTimeout(async () => {
      object = invoke<GitObject>("read_object", { path, id });
    }, 500);
  $: if (!visible) clearTimeout(t);
</script>

<div {id} bind:this={div}>
  <pre><code>{id}</code></pre>
  {#await object}
    loading...
  {:then object}
    <pre>{JSON.stringify(object, null, 2)}</pre>
  {:catch error}
    {error}
  {/await}
</div>

<style>
  div {
    margin: 1em;
    box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.1);
    border-radius: 0.2em;
    border: 1px solid grey;
    padding: 0.3em;
    height: 5em;
  }
  pre {
    font-size: 0.8em;
    background-color: aliceblue;
    border: 1px solid #ccc;
    padding: 0.3em;
    display: inline-block;
    border-radius: 0.3em;
  }
</style>
