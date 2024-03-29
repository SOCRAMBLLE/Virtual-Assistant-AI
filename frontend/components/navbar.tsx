import Link from "next/link";
import "./navbar.css";

export const NavBar = () => {
  return (
    <nav className="navbar--container">
      <Link href="/">LOGO</Link>
      <div className="navbar--navlinks">
        <Link href="/dashboard">Dashboard</Link>
        <Link href="/ai">Chat</Link>
      </div>
    </nav>
  );
};
