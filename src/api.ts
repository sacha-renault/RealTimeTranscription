import { ChatDto } from "./interfaces";

class Api {
    async getChatsByPage(page: number): Promise<ChatDto[]> {
        return [];
    }
}

export const api = new Api();