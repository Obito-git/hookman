<script lang="ts">
    import { Button, Modal, Label, Input, Checkbox } from 'flowbite-svelte';

    export let showEndpointSettingsModal: boolean;
    export let selectedEndpointId: string;
</script>

<Modal bind:open={showEndpointSettingsModal} size="md" autoclose={false} class="w-full">
    <form class="flex flex-col space-y-6" on:submit|preventDefault={() => { /* handle submission here */ }}>
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
            Endpoint Settings for {selectedEndpointId}
        </h3>
        <Label class="space-y-2">
            <span>Some Setting</span>
            <Input type="text" name="setting" placeholder="Enter setting value" required />
        </Label>
        <div class="flex items-start">
            <Checkbox>Enable Feature</Checkbox>
        </div>
        <Button type="submit" class="w-full">Save Settings</Button>
    </form>
</Modal>
