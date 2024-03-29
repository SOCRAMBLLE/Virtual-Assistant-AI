"use client";

import "./home.css";
import { Button } from "@/components/ui/button";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Home() {
  const router = useRouter();

  useEffect(() => {
    const accessToken = localStorage.getItem("accessToken");

    if (accessToken) {
      router.push("/dashboard");
    } else {
      const urlParams = new URLSearchParams(window.location.search);
      const token = urlParams.get("token");
      console.log(token);
      if (token) {
        localStorage.setItem("accessToken", token);

        router.push("/dashboard");
      }
    }
  }, []);

  const handleClick = () => {
    window.location.href = "http://127.0.0.1:8080/auth/google";
  };

  return (
    <>
      <h1 className="home-title">AI Virtual Assistant</h1>
      <div className="login-container">
        <h3 className="loginTitle">Please Login</h3>
        <Button onClick={handleClick}>Login</Button>
      </div>
    </>
  );
}
