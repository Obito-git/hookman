<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import {listen, type UnlistenFn} from '@tauri-apps/api/event';

    // Define the type of the event payload
    interface TauriEvent {
        message: string;
    }

    let events: TauriEvent[] = [];
    let unlisten: UnlistenFn;

    // Set up the event listener when the component mounts
    onMount(async () => {
        unlisten = await listen<TauriEvent>('backend-message', (event) => {
            events = [...events, event.payload];
        });
    });

    // Clean up the event listener when the component is destroyed
    onDestroy(() => {
        if (unlisten) {
            unlisten();
        }
    });
</script>

<main>
    <h1>Tauri Event Listener</h1>
    <ul>
        {#each events as event, index}
            <li>{index + 1}: {event.message}</li>
        {/each}
    </ul>
</main>
