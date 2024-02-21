"use client";

import "./home.css";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function Home() {
  const router = useRouter();
  const [postResponse, setPostResponse] = useState("");

  const handleClick = async () => {
    try {
      const response = await fetch("http://localhost:8080/auth/google", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      const data = await response.text();
      console.log(data);
      setPostResponse(data + " Please wait...");

      setTimeout(() => {
        router.push("/ai");
      }, 5000);
    } catch (error) {
      console.error("Fetch error:", error);
      setPostResponse("Login not successful");
    }
  };

  return (
    <>
      <h1 className="home-title">AI Virtual Assistant</h1>
      <div className="login-container">
        <h3 className="loginTitle">Please Login</h3>
        <Button onClick={handleClick}>Login</Button>
        <p className="post-response">{postResponse}</p>
      </div>
    </>
  );
}
