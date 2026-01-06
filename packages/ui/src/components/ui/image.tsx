import { useEffect, useState, type JSX } from "react";
import { Spinner } from "./spinner";

export function IImage({ className, ...rest }: JSX.IntrinsicElements["img"]) {
  const [loaded, setLoaded] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    const img = new Image();
    img.src = rest.src as string;
    img.onload = () => setLoaded(true);
    img.onerror = () => {
      setError(true);
      setLoaded(false);
    };
    setLoaded(false);
  }, [rest.src]);

  if (error) {
    return <div className={className + " bg-gray-200"}></div>;
  }
  if (loaded) {
    return <img className={className} {...rest} />;
  }
  return (
    <div className={className}>
      <Spinner className="absolute inset-0 m-auto h-12 w-12 text-gray-300" />
    </div>
  );
}
