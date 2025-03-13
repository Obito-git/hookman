<script lang="ts">
    let {requests, onSelectedRequestChange, selectedRequestId} = $props();
</script>

<main>
    {#each requests as request}
        <button
                class="border-2 p-2 m-1"
                class:border-red-500={request.id === selectedRequestId}
                class:border-gray-300={request.id !== selectedRequestId}
                onclick={() => onSelectedRequestChange(request.id)}>
            {request.id}: {request.http_method}: {request.host}, {request.timestamp}
        </button>
    {/each}
</main>
