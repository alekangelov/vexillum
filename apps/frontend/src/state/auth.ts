import { create } from "zustand";
import { useEffect } from "react";
import { API } from "../api";
import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router";
import { createJSONStorage, persist } from "zustand/middleware";

export const useAuth = create(
  persist<{
    token?: string;
    isAuthenticated: boolean;
    expiresAt?: number;
    login: (token: string) => void;
    logout: () => void;
  }>(
    (set) => ({
      token: undefined,
      isAuthenticated: false,
      expiresAt: undefined,
      login: async (token: string) => {
        const decoded = await API.auth
          .decodeToken({ token })
          .then((res) => res.data.data);
        if (!decoded) return;
        set({
          token,
          expiresAt: decoded?.exp * 1000,
          isAuthenticated: true,
        });
      },
      logout: async () => {
        await API.auth.logout();
        set({ token: undefined, expiresAt: undefined, isAuthenticated: false });
      },
    }),
    {
      name: "vex-auth", // name of the item in the storage (must be unique)
      storage: createJSONStorage(() => localStorage),
    }
  )
);

export const AuthProvider = (): null => {
  const { token, isAuthenticated } = useAuth();
  const { mutateAsync: refreshToken } = useMutation({
    mutationKey: ["refreshToken"],
    mutationFn: async () => {
      return (await API.auth.refreshToken()).data.data;
    },
  });
  const navigate = useNavigate();
  useEffect(() => {
    if (!token) return;
    const i = setInterval(() => {
      refreshToken().then((d) => {
        const newToken = d?.access_token;
        if (!newToken) return useAuth.getState().logout();
        useAuth.getState().login(newToken);
      });
    }, 1000 * 60 * 5); // every 5 minutes
    return () => {
      clearInterval(i);
    };
  }, [token]);

  useEffect(() => {
    if (!isAuthenticated) {
      // redirect to login page
      navigate("/auth/login");
    } else {
      // redirect to home page
      navigate("/");
    }
  }, [isAuthenticated]);
  return null;
};
