import {
  index,
  prefix,
  type RouteConfig,
  route,
} from "@react-router/dev/routes";

export default [
  index("./pages/Home/index.tsx"),
  ...prefix("project", [index("./pages/Project/index.tsx")]),
  route("splashscreen", "./pages/SplashScreen/index.tsx"),
] satisfies RouteConfig;
