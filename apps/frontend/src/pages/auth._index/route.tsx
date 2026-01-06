import { redirect } from "react-router";

export const loader = () => {
  return redirect("/auth/login");
};

export default function IndexPage() {
  return null;
}
