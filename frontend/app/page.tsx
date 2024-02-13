'use client'

import "./home.css";
import { Button } from "@/components/ui/button";
import {useRouter} from 'next/navigation' 


export default function Home() {
  const router = useRouter();
  const handleClick = () => {
    // let path = 'https://google.pt'
    // redirect(path)
    router.push('/ai')
  }

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
