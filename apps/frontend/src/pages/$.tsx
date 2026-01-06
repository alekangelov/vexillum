import { useQuery } from "@tanstack/react-query";
import { Button } from "@vexillum/ui/components/button.js";
import { data, useNavigate } from "react-router";
import { API } from "../api";

export async function loader() {
  return data(
    {},
    {
      status: 404,
      statusText: "Not Found",
    }
  );
}

export default function NotFoundPage() {
  const navigate = useNavigate();
  const { data, isLoading, error } = useQuery({
    queryKey: ["healthz"],
    queryFn: async () => {
      console.log("Fetching healthz...");
      const response = await API.health.healthz();
      console.log({ response });
      return response.data;
    },
  });
  if (isLoading) {
    return <div>Loading...</div>;
  }
  if (error) {
    return (
      <div>
        <div>Error: </div>
        <pre>
          <code>{JSON.stringify(error, null, 2)}</code>
        </pre>
      </div>
    );
  }
  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-4">404</h1>
        <p className="text-lg opacity-40">Page not found</p>
        <Button className="mt-6" onClick={() => navigate("/")}>
          Go to Home
        </Button>
        <pre>
          <code>{JSON.stringify(data, null, 2)}</code>
        </pre>
      </div>
    </div>
  );
}
