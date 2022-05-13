<script lang="ts">
  export let tree: Tree;
</script>

{#each tree.children as { id, name }}
  <a class="go" href="#{id}">{name}</a>
{/each}
