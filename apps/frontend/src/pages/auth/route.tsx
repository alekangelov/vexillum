import { Outlet } from "react-router";
import { IImage } from "@vexillum/ui/components/image.tsx";
import { Flag } from "lucide-react";

const IMAGE =
  "https://images.unsplash.com/photo-1484589065579-248aad0d8b13?q=80&w=3459&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D";

export default function AuthLayout() {
  return (
    <div className="flex min-h-screen">
      <div className="flex flex-1 min-w-1/2 flex-col justify-center px-4 py-12 sm:px-6 lg:flex-none lg:px-20 xl:px-24">
        <div className="mx-auto w-full flex-1 max-w-sm lg:w-96 flex flex-col gap-8">
          <a href="#" className="flex items-center gap-2 font-medium">
            <div className="flex h-6 w-6 items-center justify-center rounded-md bg-primary text-primary-foreground">
              <Flag className="size-4" />
            </div>
            Vexillum
          </a>
          <div className="mt-8 flex-1 flex flex-col justify-center">
            <Outlet />
          </div>
        </div>
      </div>
      {/* Right Image */}
      <div className="relative hidden w-0 flex-1 lg:block lg:w-1/2">
        <IImage
          className="absolute inset-0 h-full w-full object-cover"
          src={IMAGE}
          alt="Login background"
        />
      </div>
    </div>
  );
}
