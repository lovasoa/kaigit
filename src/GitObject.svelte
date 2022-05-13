<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Blob from "./Blob.svelte";
  import Commit from "./Commit.svelte";
import Tag from "./Tag.svelte";
  import Tree from "./Tree.svelte";
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

<div {id} bind:this={div} class="object">
  <pre><code>{id}</code></pre>
  {#await object}
    <div id="loading">loading...</div>
  {:then o}
    <pre class={o.type}><code>{o.type}</code></pre>
    <div>
      {#if o.type === "Tree"}
        <Tree tree={o} />
      {:else if o.type === "Commit"}
        <Commit commit={o} />
      {:else if o.type === "Blob"}
        <Blob blob={o} />
    {:else if o.type === "Tag"}
        <Tag tag={o} />
      {:else}
        Unknown object type: <pre>{JSON.stringify(o, null, 2)}</pre>
      {/if}
    </div>
  {:catch error}
    {error}
  {/await}
</div>

<style>
  .object {
    margin: 1em;
    box-shadow: 1px 1px 1px rgba(0, 0, 0, 0.1);
    border-radius: 0.2em;
    border: 1px solid grey;
    padding: 0.3em;
    min-height: 7em;
  }
  pre {
    font-size: 0.8em;
    background-color: aliceblue;
    border: 1px solid #ccc;
    padding: 0.3em;
    border-radius: 0.3em;
    display: inline-block;
  }
  #loading {
    text-align: center;
    padding: 2em;
  }
  .Tree {
    color: rgb(19, 59, 41);
  }
  .Blob {
    color: rgb(65, 17, 17);
  }
  .Commit {
    color: rgb(25, 63, 146);
  }
</style>
