'use client'

import { useState } from "react";
import "./chat.css";

export const ChatInput = () => {
    const [message, setMessage] = useState("");
    const [textareaHeight, setTextareaHeight] = useState("auto");

    const handleTextareaChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setMessage(event.target.value);
        const textarea = event.target;  
        textarea.style.height = '0';  
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
  return (
    <form className="input-container">
      <textarea onChange={handleTextareaChange} rows={1} cols={50} style={{ height: textareaHeight }} value={message}/>
      <button type="submit">Send</button>
    </form>
  );
};
