<script lang="ts">
    import {
        Button,
        Modal,
        Label,
        Input,
        Accordion,
        AccordionItem,
        TableHead,
        TableHeadCell,
        Table,
        TableBody,
        TableBodyRow,
        TableBodyCell,
        Helper
    } from 'flowbite-svelte';
    import {createForm} from 'felte';
    import {z} from 'zod';
    import {validator} from "@felte/validator-zod";
    import {invoke} from "@tauri-apps/api/core";
    import type {EndpointCreateDto} from "$types/endpoints";

    let {showAddEndpointModal = $bindable()} = $props();

    const schema = z.object({
        endpointUrl: z.string().nonempty("Endpoint URL is required"),
        statusCode: z.preprocess(
            (val) => {
                if (typeof val === "string" && val.trim() !== "") {
                    const num = Number(val);
                    return isNaN(num) ? val : num;
                }
                return undefined;
            },
            z.number({
                invalid_type_error: "Status code must be a number between 100 and 599"
            })
                .min(100, "Status code must be between 100 and 599")
                .max(599, "Status code must be between 100 and 599")
                .optional()
        ), headers: z.map(z.string(), z.string()),
        body: z.string().optional()
    });

    type FormData = z.infer<typeof schema>;

    const {form, data, errors} = createForm<FormData>({
        initialValues: {
            endpointUrl: '',
            statusCode: undefined,
            headers: new Map<string, string>(),
            body: ''
        },
        extend: validator({schema}),
        onSubmit: async (values) => {
            console.log("Values submitted:", values);
            const payload: EndpointCreateDto = {
                url: values.endpointUrl,
                action: {
                    customResponse: {
                        statusCode: Number(values.statusCode),
                        body: values.body,
                        headers: Object.fromEntries(values.headers.entries()),
                    }
                }
            };
            await invoke<EndpointCreateDto>('create_endpoint', {
                endpoint: payload
            }).then(() => {
                console.log("Endpoint created successfully");
                showAddEndpointModal = false;
            }).catch((error) => {
                console.error("Error creating endpoint:", error);
            });
        },
        onError: (err) => {
            console.log("Form errors:", err);
        }
    });

    let headerKey: string = $state('');
    let headerValue: string = $state('');

    function addHeader() {
        if (headerKey.trim()) {
            data.update((cur) => {
                cur.headers.set(headerKey, headerValue);
                return cur;
            })
            headerKey = '';
            headerValue = '';
        }
    }
</script>

<Modal bind:open={showAddEndpointModal} size="md" autoclose={false} class="w-full">
    <!-- Attach Felte's form action -->
    <form class="flex flex-col space-y-6 p-6" use:form>
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
            Add new endpoint
        </h3>
        <Label class="block">
            <span class="text-gray-700 dark:text-gray-300">URL</span>
            <Input
                    type="text"
                    name="endpointUrl"
                    placeholder="Enter url value"
                    required
                    class="mt-1 block w-full p-2 border rounded-md focus:outline-none focus:ring-2 "
                    color={$errors.endpointUrl ? 'red' : 'base'}
                    bind:value={$data.endpointUrl}
            />
            {#if $errors.endpointUrl}
                <Helper class="mt-2" color="red">
                    {$errors.endpointUrl}
                </Helper>
            {/if}
        </Label>
        <Accordion>
            <AccordionItem open>
                <span slot="header" class="text-gray-800 dark:text-gray-100">Custom response</span>
                <div class="space-y-4 p-4">
                    <Label class="block">
                        <span class="text-gray-700 dark:text-gray-300">Status code</span>
                        <!-- Use non-null assertion for customResponse -->
                        <Input
                                type="text"
                                name="statusCode"
                                placeholder="Enter status code"
                                class="mt-1 block w-full p-2 border rounded-md focus:outline-none focus:ring-2 "
                                bind:value={$data.statusCode}
                                color={$errors.statusCode? 'red' : 'base'}
                        />
                        {#if $errors.statusCode}
                            <Helper class="mt-2" color="red">
                                {$errors.statusCode}
                            </Helper>
                        {/if}
                    </Label>
                    <div>
                        <span class="text-gray-700 dark:text-gray-300 block mb-2">Headers</span>
                        <Table striped={true} class="min-w-full border-collapse border border-gray-300">
                            <TableHead class="border-b-2">
                                <TableHeadCell>Key</TableHeadCell>
                                <TableHeadCell>Value</TableHeadCell>
                            </TableHead>
                            <TableBody tableBodyClass="divide-y">
                                {#each $data.headers.entries() as [key, value]}
                                    <TableBodyRow>
                                        <TableBodyCell>{key}</TableBodyCell>
                                        <TableBodyCell>{value}</TableBodyCell>
                                    </TableBodyRow>
                                {/each}
                            </TableBody>
                        </Table>
                        <div class="flex items-center gap-2 mt-4">
                            <Input
                                    type="text"
                                    name="key"
                                    placeholder="Key"
                                    bind:value={headerKey}
                                    class="flex-1 p-2 border rounded-md focus:outline-none focus:ring-2"
                            />
                            <Input
                                    type="text"
                                    name="value"
                                    placeholder="Value"
                                    bind:value={headerValue}
                                    class="flex-1 p-2 border rounded-md focus:outline-none focus:ring-2"
                            />
                            <Button
                                    on:click={addHeader}
                                    type="button"
                                    class="px-4 py-2 text-white rounded-md"
                            >
                                Add
                            </Button>
                        </div>
                    </div>
                    <Label class="block">
                        <span class="text-gray-700 dark:text-gray-300">Body</span>
                        <!-- Use non-null assertion for customResponse.body -->
                        <Input
                                type="text"
                                name="body"
                                placeholder="Enter body value"
                                class="mt-1 block w-full p-2 border rounded-md focus:outline-none focus:ring-2"
                                bind:value={$data.body}
                        />
                        {#if $errors.body}
                            <p class="mt-1 text-sm text-red-600">{$errors.body}</p>
                        {/if}
                    </Label>
                </div>
            </AccordionItem>
        </Accordion>
        <Button type="submit" class="w-full px-4 py-2 text-white rounded-md">
            Save
        </Button>
    </form>
</Modal>
