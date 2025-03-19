<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import type {WebhookRequest} from "$types/requests";

    let {selectedRequestId, requestCache = $bindable()} = $props();

    let request: WebhookRequest | undefined = $state(undefined);

    $effect(() => {
        if (selectedRequestId) {
            if (requestCache[selectedRequestId]) {
                request = requestCache[selectedRequestId];
            } else {
                (async () => {
                    const fetched = await getRequest();
                    if (fetched) {
                        requestCache[selectedRequestId] = fetched;
                        request = fetched;
                    }
                })();
            }
        } else {
            request = undefined;
        }
    });

    async function getRequest() {
        if (selectedRequestId) {
            return invoke<WebhookRequest>('get_request', {requestId: selectedRequestId})
                .catch((error) => {
                    console.error("Error fetching data:", error);
                    return undefined;
                });
        }
        return undefined;
    }

</script>

<main>
    {#if request}
        <div>Request ID: {selectedRequestId}</div>
        <div>Timestamp: {request.timestamp}</div>
        <div>Host: {request.host}</div>
        <div>HTTP Method: {request.method}</div>
        <div>Headers:</div>
        <ul>
            {#each request.headers as header}
                <li>{header.key}: {header.value}</li>
            {/each}
        </ul>
        <div>Query Params:</div>
        <ul>
            {#each request.queryParams as param}
                <li>{param.key}: {param.value}</li>
            {/each}
        </ul>
        <div>Body: {request.body}</div>
    {/if}
</main>