import { useMutation } from "@tanstack/react-query";
import { Button } from "@vexillum/ui/components/button.tsx";
import { Input } from "@vexillum/ui/components/input.tsx";
import { Label } from "@vexillum/ui/components/label.tsx";
import { cn } from "@vexillum/ui/lib/utils.js";
import { API } from "../../api";
import { useAuth } from "../../state/auth";

export default function LoginRoute() {
  const { isPending, mutateAsync: login } = useMutation({
    mutationKey: ["login"],
    mutationFn: async (credentials: { email: string; password: string }) => {
      return (await API.auth.login(credentials)).data.data;
    },
  });
  const state = useAuth();
  return (
    <form
      className={cn("flex flex-col gap-6")}
      onSubmit={async (e) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget);
        const { access_token: aT } =
          (await login({
            email: formData.get("email") as string,
            password: formData.get("password") as string,
          })) ?? {};
        if (!aT) return;
        state.login(aT);
      }}
    >
      <title>Login</title>
      <div className="flex flex-col items-center gap-2 text-center">
        <h1 className="text-2xl font-bold">Login to your account</h1>
        <p className="text-balance text-sm text-muted-foreground">
          Enter your email below to login to your account
        </p>
      </div>
      <div className="grid gap-6">
        <div className="grid gap-2">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            type="email"
            name="email"
            placeholder="me@example.com"
            required
          />
        </div>
        <div className="grid gap-2">
          <div className="flex items-center">
            <Label htmlFor="password">Password</Label>
            <a
              href="/auth/magic"
              className="ml-auto text-sm underline-offset-4 hover:underline"
            >
              Login via magic link?
            </a>
          </div>
          <Input id="password" name="password" type="password" required />
        </div>
        <Button disabled={isPending} type="submit" className="w-full">
          Login
        </Button>
      </div>
    </form>
  );
}
