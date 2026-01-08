import { useLocation } from "react-router";
import { PAGE_TITLES } from "./titles";
export const useBreadcrumbs = (): Array<{
  href: string;
  label: string;
}> => {
  const location = useLocation();
  const pathnames = location.pathname.split("/").filter((x) => x);
  const breadcrumbs: Array<{ href: string; label: string }> = [];
  let accumulatedPath = "";
  pathnames.forEach((_, index) => {
    accumulatedPath += `/${pathnames[index]}`;
    const label = PAGE_TITLES[accumulatedPath] || pathnames[index];
    breadcrumbs.push({ href: accumulatedPath, label });
  });
  return breadcrumbs;
};
