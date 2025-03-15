<script lang="ts">
    import {Button, Modal, Label, Input, Checkbox} from 'flowbite-svelte';

    let {showAddEndpointModal = $bindable()} = $props();
    let isCustomizeResponseChecked = $state(false);
</script>

<Modal bind:open={showAddEndpointModal} size="md" autoclose={false} class="w-full">
    <form class="flex flex-col space-y-6" on:submit|preventDefault={() => { /* handle submission here */ }}>
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
            Add new endpoint
        </h3>
        <Label class="space-y-2">
            <span>Some Setting</span>
            <Input type="text" name="setting" placeholder="Enter setting value" required/>
        </Label>
        <div class="flex items-start">
            <Checkbox bind:checked={isCustomizeResponseChecked}>Customize response</Checkbox>
        </div>
        {#if isCustomizeResponseChecked}
            <Label class="space-y-2">
                <span>Status code</span>
                <Input type="text" name="setting" placeholder="Enter setting value" required/>
            </Label>
            <Label class="space-y-2">
                <span>Headers</span>
                <Input type="text" name="setting" placeholder="Enter setting value" required/>
            </Label>
            <Label class="space-y-2">
                <span>Body</span>
                <Input type="text" name="setting" placeholder="Enter setting value" required/>
            </Label>
        {/if}
        <Button type="submit" class="w-full">Save</Button>
    </form>
</Modal>
