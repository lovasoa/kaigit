<script lang="ts">
  import GitRepo from "./GitRepo.svelte";
  let path: string = "/home/olojkine/dev/candilib-auto/";
</script>

<header class:haspath={path}>
  <h1>Kaigit</h1>
  <input bind:value={path} placeholder="Path to local git directory folder" />
</header>

{#if path}
  <GitRepo {path} />
{/if}

<style>
  header {
    text-align: center;
    padding: 2em;
    max-width: 600px;
    margin: 0 auto;
    font-size: 2em;
    line-height: 1.5em;
  }
  header input {
    width: 100%;
    flex: 1;
    font-family: monospace;
  }
  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-weight: 100;
    padding: 0.3em;
    margin: 0.1em 1em;
  }

  header.haspath {
    margin: 0 0;
    padding: 0.6em;
    display: flex;
    font-size: 1em;
    line-height: 1em;
    max-width: none;
    border-bottom: 1px solid grey;
  }
</style>
