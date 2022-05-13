<script lang="ts">
  export let tag: Tag;
</script>

<a class="go" href="#{tag.target}" id={tag.name}>{tag.name}</a>
