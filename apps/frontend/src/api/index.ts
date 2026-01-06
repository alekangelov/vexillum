import { HealthApi, AuthenticationApi, Configuration } from "@vexillum/reqx";
import axios from "axios";
import { createContext } from "react";

const BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";
const axiosInstance = axios.create({
  headers: {
    "Content-Type": "application/json",
  },
});

const config = new Configuration({});

export const API = {
  health: new HealthApi(config, BASE_URL.replace("/api", ""), axiosInstance),
  auth: new AuthenticationApi(config, BASE_URL, axiosInstance),
};
console.log({
  BASE_URL,
  API,
});

const ApiContext = createContext({});

export const ApiProvider = ({ children }: { children: React.ReactNode }) => {
  return children;
};
