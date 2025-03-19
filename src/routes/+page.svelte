<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";
    import RequestDetails from "$lib/RequestDetails.svelte";
    import type {Endpoint} from "$types/endpoints";
    import type {WebhookRequest, WebhookRequestPreview} from "$types/requests";
    import {Button, Indicator, TabItem, Tabs} from "flowbite-svelte";
    import AddIcon from 'virtual:icons/mingcute/classify-add-2-line';
    import {type EndpointReadDto, type NotifyMessage, NotifyMessageEvent} from "$types/events";
    import EndpointAddModal from "$lib/EndpointAddModal.svelte";
    import {SvelteMap} from "svelte/reactivity";

    interface EndpointTabDetails {
        url: string;
        unreadCount: number;
    }

    let endpoints: SvelteMap<string, EndpointTabDetails> = new SvelteMap();

    let selectedEndpointId: string | undefined = $state(undefined);
    let selectedRequestId: string | undefined = $state(undefined);
    const requestCache: { [id: string]: WebhookRequest } = {};
    let requests: WebhookRequestPreview[] = $state([]);
    let showAddEndpointModal = $state(false)

    $effect(() => {
        if (selectedEndpointId) {
            (async () => {
                requests = await getRequests();
                if (requests.length > 0) {
                    selectedRequestId = requests[0].id; // TODO: Handle empty
                }
            })();
        }
    });

    async function getRequests() {
        if (selectedEndpointId) {
            return invoke<WebhookRequestPreview[]>('get_requests_by_endpoint_id', {endpointId: selectedEndpointId})
                .catch((error) => {
                    console.error("Error fetching data:", error);
                    return [];
                });
        }
        return [];
    }

    let unlisten: UnlistenFn;

    onMount(async () => {
        const endpoints_list: Endpoint[] = await invoke("get_endpoints");
        if (endpoints_list.length !== 0) {
            selectedEndpointId = endpoints_list[0].id;
            endpoints_list.forEach((endpoint => {
                endpoints.set(endpoint.id, {
                    url: endpoint.url,
                    unreadCount: 0
                });
            }));
        }
        unlisten = await listen<NotifyMessage>('backend-message', (event) => {
            console.log('Received event:', event.payload.event);
            switch (event.payload.event) {
                case NotifyMessageEvent.EndpointAdded:
                    console.log('Endpoint added:', event.payload.data);
                    const endpointData: EndpointReadDto = event.payload.data;
                    const endpoint: Endpoint = {
                        id: endpointData.id,
                        url: endpointData.url,
                    };
                    if (!selectedEndpointId) {
                        selectedEndpointId = endpoint.id;
                    }
                    endpoints.set(endpoint.id, {
                        url: endpoint.url,
                        unreadCount: 0
                    });
                    break;
                case NotifyMessageEvent.EndpointRemoved:
                    console.log('Endpoint removed:', event.payload.data);
                    break;
                case NotifyMessageEvent.RequestAdded:
                    const requestData: WebhookRequestPreview = event.payload.data;
                    if (requestData.endpointId === selectedEndpointId) {
                        requests.unshift(requestData);
                    } else {
                        // Get the current endpoint data
                        const endpointData = endpoints.get(requestData.endpointId);
                        if (endpointData) {
                            endpoints.set(requestData.endpointId, {
                                ...endpointData,
                                unreadCount: endpointData.unreadCount + 1,
                            });
                        }
                    }
                    break;
                default:
                    console.error('Unknown event');
            }
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
            const endpointData = endpoints.get(id);
            if (endpointData) {
                endpoints.set(id, {
                    ...endpointData,
                    unreadCount: 0,
                });
            }
        }
    }
</script>

<main class="container mx-auto min-h-screen flex flex-col">
    <Tabs>
        <!-- Add New Endpoint Button -->
        <button
                onclick={() => showAddEndpointModal = true}
                class="p-2 hover:bg-gray-200 rounded-full"
        >
            <AddIcon class="w-8 h-8"/>
        </button>
        {#each endpoints as endpoint}
            <!-- Endpoint tab -->
            <TabItem open={selectedEndpointId === endpoint[0]} on:click={() => handleEndpointClick(endpoint[0])}>
                <div slot="title" class="relative pr-4">
                    {endpoint[1].url}
                    {#if endpoint[1].unreadCount > 0}
                        <Indicator color="red" border size="xl" placement="top-right">
                            <span class="text-white text-xs font-bold">{endpoint[1].unreadCount}</span>
                        </Indicator>
                    {/if}
                </div>
                <div class="flex h-screen">
                    <div class="w-1/4 border-r">
                        <!-- Request tab -->
                        <Tabs tabStyle="underline" class="flex flex-col">
                            {#each requests as request}
                                <TabItem open={selectedRequestId === request.id}
                                         on:click={() => handleRequestClick(request.id)}>
                                    <div slot="title" class="flex items-center gap-2">
                                        {request.id}: {request.method}: {request.host}, {request.timestamp}
                                    </div>
                                </TabItem>
                            {/each}
                        </Tabs>
                    </div>

                    <!-- Request details -->
                    <div class="flex-1 p-4">
                        <RequestDetails {selectedRequestId} {requestCache}/>
                    </div>
                </div>
            </TabItem>
        {/each}
    </Tabs>
    <EndpointAddModal bind:showAddEndpointModal/>
</main>

