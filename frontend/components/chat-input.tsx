"use client";

import React, { useState } from "react";
import "./chat.css";


export const ChatInput = () => {
  const [message, setMessage] = useState("");
  const [textareaHeight, setTextareaHeight] = useState("auto");

  const handleTextareaChange = (
    event: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    setMessage(event.target.value);
    const textarea = event.target;
    textarea.style.height = "0";
    const newHeight = textarea.scrollHeight;
    const maxHeight = 110;
    if (newHeight > maxHeight) {
      textarea.style.height = `${maxHeight}px`;
      setTextareaHeight(`${maxHeight}px`);
    } else {
      textarea.style.height = `${newHeight}px`;
      setTextareaHeight(`${newHeight}px`);
    }
  };

  const handleInputSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    try {
      const response = await fetch('/api/chat', {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ message }),
      });
      if (!response.ok) {
        throw new Error(`Failed to fetch, status: ${response.status}`);
      }
      const data = await response.text();
      console.log("Mensagem enviada:", message);
      console.log("Resposta:", data);
    } catch (error) {
      console.error("Fetch error:", error);
    }
  
  };

  return (
    <form className="input-container" onSubmit={handleInputSubmit}>
      <textarea
        onChange={handleTextareaChange}
        rows={1}
        cols={50}
        style={{ height: textareaHeight }}
        value={message}
      />
      <button type="submit">Send</button>
    </form>
  );
};
