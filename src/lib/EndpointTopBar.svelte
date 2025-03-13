<script lang="ts">
    let {endpoints} = $props();
</script>

<main>
    <h1>Endpoints:</h1>
    <ul>
        {#each endpoints as event}
            <li> {event.id}: {event.uri} </li>
        {/each}
    </ul>
</main>
