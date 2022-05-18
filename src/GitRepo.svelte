<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onDestroy } from "svelte";
  import GitObject from "./GitObject.svelte";
  export let path: string;
  let observer: IntersectionObserver | null = null;
  const visible: { [id: string]: boolean } = {};
  $: git_result = invoke<GitRepo>("git_data", { path });
  observer = new IntersectionObserver((entries) => {
    for (const entry of entries)
      visible[entry.target.id] = entry.isIntersecting;
  });
  onDestroy(() => observer.disconnect());

  function resolve_ref(all_refs: Reference[], ref: Reference) {
    for (let i = 0; ref.is_symbolic && i < 99; i++)
      ref = all_refs.find((r) => r.name === ref.target);
    return ref.target;
  }
</script>

{#await git_result}
  <div class="loading">Loading...</div>
{:then git_data}
  <i><b>{git_data.objects.length}</b> objects in the reposity.</i>
  <div class="references">
    <i>References</i>
    {#each git_data.references as ref}
      <a class="go" href="#{resolve_ref(git_data.references, ref)}" name={ref.target}
        >{ref.name}{#if ref.is_symbolic} → {ref.target}{/if}</a
      >
    {/each}
  </div>
  <div class="objects">
    {#each git_data.objects as id}
      <GitObject {id} {observer} visible={visible[id]} {path} />
    {/each}
  </div>
{:catch error}
  <div class="error">{error}</div>
{/await}

<style>
  .loading {
    text-align: center;
    padding: 2em;
  }
  .error {
    background-color: rgb(249, 157, 157);
    border: 2px solid rgb(192, 13, 13);
    padding: 1em;
    margin: 1em;
    border-radius: 0.5em;
  }
  .references {
    background-color: aliceblue;
    border: 1px solid grey;
  }
  .references a {
    background-color: rgb(207, 207, 207);
    padding: 0.2em;
  }
  i {
    color: grey;
    margin: 1em;
    display: block;
  }
</style>
