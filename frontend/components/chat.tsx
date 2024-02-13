"use client";

import { useState } from "react";
import { ChatInput } from "./chat-input";
import { ChatMessages } from "./chat-messages";
import "./chat.css";

export const Chat = () => {
  const [messages, setMessages] = useState<{ role: string; content: string }[]>(
    []
  );
  const [userMessage, setUserMessage] = useState("");

  const handleInputSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      const newUserMessage = { role: "User", content: userMessage };
      setMessages((prevMessages) => [...prevMessages, newUserMessage]);
      const response = await fetch("/api/chat", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ userMessage }),
      });
      if (!response.ok) {
        throw new Error(`Failed to fetch, status: ${response.status}`);
      }
      const data = await response.text();
      const newAiMessage = { role: "Virtual Assistant", content: data };
      setMessages((prevMessages) => [...prevMessages, newAiMessage]);
      setUserMessage("");
    } catch (error) {
      console.error("Fetch error:", error);
    }
  };

  return (
    <>
      <div className="chat-container">
        <ChatMessages messages={messages} />
        <ChatInput
          onSubmit={handleInputSubmit}
          setMessage={setUserMessage}
          message={userMessage}
        />
      </div>
    </>
  );
};
