import LogoutBtn from "@/components/logout-button";
import { useRouter } from "next/router";

export default function DashboardPage() {
  return (
    <>
      <h1>Dashboard Page</h1>
      <LogoutBtn />
    </>
  );
}
