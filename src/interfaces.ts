export interface ChatDto {
    id: number;
    title: string;
    description: string | null;
    date: Date;
}

export interface NewChatDto {
    title: string;
    description: string | null;
}