<script lang="ts">
    import GearIcon from 'virtual:icons/mingcute/settings-7-line';
    import AddIcon from 'virtual:icons/mingcute/classify-add-2-line';
    import EndpointSettingsModal from "$lib/EndpointSettingsModal.svelte";
    import EndpointAddModal from "$lib/EndpointAddModal.svelte";

    let {endpoints, onSelectedEndpointChange, selectedEndpointId} = $props();
    let showEndpointSettingsModal = $state(false)
    let showAddEndpointModal = $state(false)
</script>

<main class="flex flex-wrap gap-2">
    <button
            onclick={() => showAddEndpointModal = true}
            class="p-2 hover:bg-gray-200 rounded-full"
    >
        <AddIcon class="w-8 h-8"/>
    </button>
    {#each endpoints as endpoint}
        <div
                role="button"
                tabindex="0"
                class="inline-flex items-center justify-between border-2 rounded-lg p-3 cursor-pointer transition-colors hover:bg-gray-100"
                class:border-red-500={endpoint.id === selectedEndpointId}
                class:border-gray-300={endpoint.id !== selectedEndpointId}
                onclick={() => onSelectedEndpointChange(endpoint.id)}
                onkeydown={() => {}}
        >
            <span class="text-base font-medium">{endpoint.url}</span>
            <button
                    onclick={() => showEndpointSettingsModal = true}
                    class="p-2 hover:bg-gray-200 rounded-full"
            >
                <GearIcon class="w-5 h-5"/>
            </button>
        </div>
    {/each}

    <EndpointAddModal bind:showAddEndpointModal/>
    <EndpointSettingsModal bind:showEndpointSettingsModal selectedEndpointId={selectedEndpointId}/>
</main>

