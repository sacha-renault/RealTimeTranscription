import { ChatDto, NewChatDto, ChatMessageDto } from "./interfaces";

class Api {
    async getChatsByPage(page: number): Promise<ChatDto[]> {
        return [{
            title: "zob",
            description: "Il était une fois un tout petit zob. Ce zob allait en cours d'histoire économique et il était content. FIN.",
            date: new Date(Date.now()),
            id: page
        }];
    }

    // create a new chat and in case of success it will return the id of the chat
    async createNewChat(newChat: NewChatDto): Promise<number> {
        return 0;
    }

    async getChatById(chatId: number): Promise<ChatDto> {
        return {
            title: "zob",
            description: "Il était une fois un tout petit zob. Ce zob allait en cours d'histoire économique et il était content. FIN.",
            date: new Date(Date.now()),
            id: chatId
        };
    }

    async getMessageByChatId(chatId: number): Promise<ChatMessageDto[]> {
        return [{
            content: "message: " + chatId,
            id: 0,
            date: new Date(Date.now())
        }]
    }
}

export const api = new Api();