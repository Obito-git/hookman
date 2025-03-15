<script lang="ts">
    import RequestSideBar from "$lib/RequestSideBar.svelte";
    import EndpointTopBar from "$lib/EndpointTopBar.svelte";
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";
    import RequestDetails from "$lib/RequestDetails.svelte";
    import type {Endpoint} from "$types/endpoints";
    import type {TauriEvent} from "$types/events";
    import type {WebhookRequestPreview} from "$types/requests";


    let endpoints: Endpoint[] = $state([]);
    let selectedEndpointId: string | undefined = $state(undefined);
    let selectedRequestId: string | undefined = $state(undefined);
    let requests: WebhookRequestPreview[] = $state([]);

    $effect(() => {
        if (selectedEndpointId) {
            (async () => {
                requests = await getRequests();
            })();
        }
    });

    async function getRequests() {
        if (selectedEndpointId) {
            return invoke<WebhookRequestPreview[]>('get_requests_by_endpoint_id', {selectedEndpointId})
                .catch((error) => {
                    console.error("Error fetching data:", error);
                    return [];
                });
        }
        return [];
    }

    let unlisten: UnlistenFn;

    onMount(async () => {
        endpoints = await invoke("get_endpoints", {endpoints});
        selectedEndpointId = endpoints[0].id; // TODO: Handle empty
        unlisten = await listen<TauriEvent>('backend-message', (event) => {
            const newRequest: WebhookRequestPreview = {
                id: event.payload.id,
                http_method: event.payload.method,
                host: event.payload.host,
                timestamp: event.payload.date,
            }
            requests.unshift(newRequest);
        });
    });

    onDestroy(() => {
        if (unlisten) {
            unlisten();
        }
    });

    function handleRequestClick(id: string) {
        selectedRequestId = id;
    }

    function handleEndpointClick(id: string) {
        if (selectedEndpointId !== id) {
            selectedEndpointId = id;
            requests = [];
            selectedRequestId = undefined;
        }
    }
</script>

<main class="container mx-auto min-h-screen flex flex-col">
    <!-- Top bar -->
    <div class="mb-4">
        <EndpointTopBar onSelectedEndpointChange={handleEndpointClick} {endpoints} {selectedEndpointId}/>
    </div>

    <!-- Split content: sidebar and details -->
    <div class="flex flex-1">
        <aside class="w-1/4 p-4 border-r border-gray-300">
            <RequestSideBar onSelectedRequestChange={handleRequestClick} {requests} {selectedRequestId}/>
        </aside>

        <section class="w-3/4 p-4">
            <div class="border-t-4">
                <RequestDetails {selectedRequestId}/>
            </div>
        </section>
    </div>
</main>

