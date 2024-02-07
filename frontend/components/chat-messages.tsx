import "./chat.css";
import { FaRocketchat, FaRegUser  } from "react-icons/fa";

export const ChatMessages = () => {
  return (
    <div className="chat-messages-container">
        <div className="message-container">
            <FaRegUser />
            <div className="message-subcontainer">
                <p className="message-role">User</p>
                <p className="message-content">Olá meu amigo!</p>
            </div>
        </div>
        <div className="message-container">
            <FaRocketchat />
            <div className="message-subcontainer">
                <p className="message-role">Virtual Assistant</p>
                <p className="message-content">Olá eu sou o teu Virtual Assistant!</p>
            </div>
        </div>
    </div>
  );
};
