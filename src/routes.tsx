import {
  index,
  layout,
  prefix,
  type RouteConfig,
  route,
} from "@react-router/dev/routes";

export default [
  index("./pages/Home/index.tsx"),
  ...prefix("project", [
    layout("./pages/Project/layout/index.tsx", [
      index("./pages/Project/index.tsx"),
    ]),
  ]),
  route("splashscreen", "./pages/SplashScreen/index.tsx"),
] satisfies RouteConfig;
