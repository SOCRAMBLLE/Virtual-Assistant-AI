import "./chat.css";
import { FaRocketchat, FaRegUser } from "react-icons/fa";

interface ChatMessagesProps {
  messages: { role: string; content: string }[];
}

export const ChatMessages: React.FC<ChatMessagesProps> = ({ messages }) => {
  return (
    <div className="chat-messages-container">
      {messages.map((message, index) => (
        <div className="message-container" key={index}>
          {message.role === "User" ? <FaRegUser /> : <FaRocketchat />}
          <div className="message-subcontainer">
            <p className="message-role">{message.role}</p>
            <p className="message-content">{message.content}</p>
          </div>
        </div>
      ))}
    </div>
  );
};
