import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useAuth } from "../state/auth";

export default function IndexPage() {
  const navigate = useNavigate();
  const { isAuthenticated } = useAuth();

  useEffect(() => {
    if (isAuthenticated) {
      navigate("/dashboard");
    } else {
      navigate("/auth/login");
    }
  }, [navigate, isAuthenticated]);
}
