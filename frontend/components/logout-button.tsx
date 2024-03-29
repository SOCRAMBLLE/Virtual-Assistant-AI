"use client";
import { useRouter } from "next/router";
import { Button } from "./ui/button";
import { useEffect } from "react";

const LogoutBtn = () => {
  const router = useRouter();

  // useEffect(() => {
  //   const accessToken = localStorage.getItem("accessToken");
  //   if (!accessToken) {
  //     router.push("/");
  //   }
  // }, [router]);

  // const handleLogout = () => {
  //   localStorage.removeItem("accessToken");
  //   router.push("/");
  // };

  return <Button>Logout</Button>;
};

export default LogoutBtn;
