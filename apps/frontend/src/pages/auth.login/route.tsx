import { Button } from "@vexillum/ui/components/button.tsx";
import { Input } from "@vexillum/ui/components/input.tsx";
import { Label } from "@vexillum/ui/components/label.tsx";
import { cn } from "@vexillum/ui/lib/utils.js";

export default function LoginRoute() {
  return (
    <form className={cn("flex flex-col gap-6")}>
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
          <Input id="email" type="email" placeholder="m@example.com" required />
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
          <Input id="password" type="password" required />
        </div>
        <Button type="submit" className="w-full">
          Login
        </Button>
      </div>
      {/* <div className="text-center text-sm">
        Don&apos;t have an account?{" "}
        <a href="#" className="underline underline-offset-4">
          Sign up
        </a>
      </div> */}
    </form>
  );
}
