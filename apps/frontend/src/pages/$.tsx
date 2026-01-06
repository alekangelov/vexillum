import { Button } from "@vexillum/ui/components/button.js";
import { data } from "react-router";

export async function loader() {
  return data("Not Found", {
    status: 404,
    statusText: "Not Found",
  });
}

export default function NotFoundPage() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-4">404</h1>
        <p className="text-lg opacity-40">Page not found</p>
        <Button className="mt-6">Go to Home</Button>
      </div>
    </div>
  );
}
