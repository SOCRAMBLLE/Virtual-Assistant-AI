"use client";
import { useRouter } from "next/router";
import { Button } from "./ui/button";
import { useEffect } from "react";
import "./logout-button.css";

const LogoutBtn = () => {
  const handleLogout = () => {
    localStorage.removeItem("accessToken");
  };

  return (
    <Button onClick={handleLogout} className="logout-btn">
      Logout
    </Button>
  );
};

export default LogoutBtn;
