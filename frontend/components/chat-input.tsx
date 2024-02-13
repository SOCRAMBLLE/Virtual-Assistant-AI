"use client";

import React, { useState, useEffect, useRef } from "react";
import "./chat.css";
import { Button } from "./ui/button";
import { IoMdSend } from "react-icons/io";

interface ChatProps {
  onSubmit: (event: React.FormEvent<HTMLFormElement>) => void;
  setMessage: React.Dispatch<React.SetStateAction<string>>;
  message?: string;  
}

export const ChatInput: React.FC<ChatProps> = ({ onSubmit, setMessage, message }) => {
  const [textareaHeight, setTextareaHeight] = useState("auto");
  const [isMessageEmpty, setIsMessageEmpty] = useState(true);
  const [isTouchDevice, setIsTouchDevice] = useState(false);
  const myFormRef = useRef<HTMLFormElement | null>(null);

  useEffect(() => {
    setIsMessageEmpty(!message);
  }, [message]);

  useEffect(() => {
    const onTouchStart = () => setIsTouchDevice(true);
    window.addEventListener("touchstart", onTouchStart);

    return () => window.removeEventListener("touchstart", onTouchStart);
  }, []);

  const pressEnter = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey && !isTouchDevice) {
      e.preventDefault();
      if (!isMessageEmpty) {
        myFormRef.current?.requestSubmit();
      }
    }
  };

  const handleTextareaChange = (
    event: React.ChangeEvent<HTMLTextAreaElement>
  ) => {
    message = event.target.value
    setMessage(message);
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

  return (
    <form className="input-container" onSubmit={(e) => !isMessageEmpty && onSubmit(e)} ref={myFormRef}>
      <textarea
        placeholder="Type a message..."
        onChange={handleTextareaChange}
        rows={1}
        cols={50}
        style={{ height: textareaHeight }}
        value={message}
        onKeyDown={pressEnter}
      />
      <Button type="submit" className={isMessageEmpty ? "inactive": ""}><IoMdSend /></Button>
    </form>
  );
};
