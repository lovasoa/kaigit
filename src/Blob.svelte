<script lang="ts">
    export let blob: GitBlob;
</script>

<pre>{blob.short_contents}</pre>

<style>
    pre{
        height: 6.5em;
        overflow: scroll;
        border: 1px solid grey;
    }
</style>