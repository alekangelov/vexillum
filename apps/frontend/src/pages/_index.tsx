import { Button } from "@vexillum/ui/components/button.js";
import { useAuth } from "../state/auth";

export default function IndexPage() {
  return (
    <div>
      Welcome to the Vexillum Frontend!
      <Button
        onClick={() => {
          useAuth.getState().logout();
        }}
      >
        Logout
      </Button>
    </div>
  );
}
