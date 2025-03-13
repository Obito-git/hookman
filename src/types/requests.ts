export interface WebhookRequest {
    headers: KeyValue[];
    query_params: KeyValue[];
    body: string;
    timestamp: string;
    host: string;
    http_method: string;
}

export interface WebhookRequestPreview {
    id: string;
    http_method: string;
    host: string;
    timestamp: string;
}

export interface KeyValue {
    key: string;
    value: string;
}