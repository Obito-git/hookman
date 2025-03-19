export interface WebhookRequest {
    headers: KeyValue[];
    queryParams: KeyValue[];
    body: string;
    timestamp: string;
    host: string;
    method: string;
}

export interface WebhookRequestPreview {
    timestamp: string;
    host: string;
    method: string;
    endpointId: string;
    id: string;
}

export interface KeyValue {
    key: string;
    value: string;
}