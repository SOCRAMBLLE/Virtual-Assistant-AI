import type { Metadata } from "next";
import "./globals.css";
import { NavBar } from "@/components/navbar";

export const metadata: Metadata = {
  title: "Virtual Assistant",
  description: "AI Virtual Assistant",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <main className="body--container">
          <NavBar />
          {children}
        </main>
      </body>
    </html>
  );
}
