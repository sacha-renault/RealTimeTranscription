import { ChatDto, NewChatDto, ChatMessageDto } from "./interfaces";
import { invoke } from "@tauri-apps/api/core";

class Api {
    async getChatsByPage(page: number): Promise<ChatDto[]> {
        const result: ChatDto[] = await invoke('get_chat_by_page', { pageNum: page, pageSize: 10 });
        return result;
    }

    // create a new chat and in case of success it will return the id of the chat
    async createNewChat(newChat: NewChatDto): Promise<number> {
        const result: number = await invoke('create_new_chat',
            {
                title: newChat.title,
                description: newChat.description
            }
        );
        return result;
    }

    async getChatById(chatId: number): Promise<ChatDto> {
        const result: ChatDto = await invoke('get_chat_by_id', { id: chatId });
        return result;
    }

    async getMessageByChatId(chatId: number): Promise<ChatMessageDto[]> {
        const result: ChatMessageDto[] = await invoke('get_messages_by_chat_id', { chatId: chatId });
        return result;
    }

    async addMessage(message: string, chatId: number): Promise<void> {
        await invoke('add_new_message', { chatId: chatId, content: message });
    }
}

export const api = new Api();