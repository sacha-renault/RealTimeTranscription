export interface ChatDto {
    id: number;
    title: string;
    description: string | null;
    date: string;
}

export interface NewChatDto {
    title: string;
    description: string | null;
}

export interface ChatMessageDto {
    date: string;
    content: string;
    id: number;
    chatId: number;
}