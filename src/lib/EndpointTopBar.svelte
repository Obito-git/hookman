<script lang="ts">
    import GearIcon from 'virtual:icons/mingcute/settings-7-line';
    import {Button, Modal} from 'flowbite-svelte';
    import MyModal from "$lib/MyModal.svelte";

    let { endpoints, onSelectedEndpointChange, selectedEndpointId, onGearIconClick } = $props();
    let defaultModal = false;
</script>

<main class="flex flex-wrap gap-2">
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
            <span class="text-base font-medium">{endpoint.uri}</span>
            <button
                    type="button"
                    onclick={() => defaultModal = true}
                    class="p-2 hover:bg-gray-200 rounded-full"
            >
                <GearIcon class="w-5 h-5" />
            </button>
        </div>
    {/each}
    <MyModal/>
</main>

