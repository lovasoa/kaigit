<script lang="ts">
  export let commit: Commit;
</script>

{#each commit.parents as parent, i}
  <a class="go" href="#{parent}">Parent {i + 1}</a>
{:else}
  <span>No parents</span>
{/each}
<a class="go" href="#{commit.tree}">Tree</a>

<p>
  message: <code>{commit.message}</code>
</p>
<p>
  By <i>{commit.author}</i>
</p>
