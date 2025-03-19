export interface Endpoint {
    id: string,
    url: string,
}

export interface EndpointCreateDto {
    url: string;
    action?: {
        customResponse: CustomResponseSettings;
    };
}

export interface CustomResponseSettings {
    statusCode?: number;
    body?: string;
    headers?: Record<string, string>;
}



