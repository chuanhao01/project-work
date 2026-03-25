import {
  type RouteConfig,
  index,
  route,
} from "@react-router/dev/routes";


export default [
  index("./pages/Home/index.tsx"),
  route("splashscreen", "./pages/SplashScreen/index.tsx"),
] satisfies RouteConfig;
