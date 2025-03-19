import type {WebhookRequestPreview} from "$types/requests";

export enum NotifyMessageEvent {
    EndpointAdded = 'endpointAdded',
    EndpointRemoved = 'endpointRemoved',
    RequestAdded = 'requestAdded',
}

export type NotifyMessage =
    | {
    event: NotifyMessageEvent.EndpointAdded;
    data: EndpointReadDto;
}
    | {
    event: NotifyMessageEvent.EndpointRemoved;
    data: EndpointReadDto;
}
    | {
    event: NotifyMessageEvent.RequestAdded;
    data: WebhookRequestPreview;
};

export interface EndpointReadDto {
    id: string;
    url: string;
    action?: EndpointAction;
}


export type EndpointAction =
    | { customResponse: CustomResponseSettings }
    | { none: {} };

export interface CustomResponseSettings {
    statusCode: number;
    body: string;
    headers: any;
}
