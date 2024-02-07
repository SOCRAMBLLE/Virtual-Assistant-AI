import { ChatInput } from "./chat-input"
import { ChatMessages } from "./chat-messages"
import "./chat.css"

export const Chat = () => {
    return (
        <>
            <div className="chat-container">
                <ChatMessages />
                <ChatInput />
            </div>
        </>
    )
}