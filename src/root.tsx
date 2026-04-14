import { listen } from "@tauri-apps/api/event";
import { useState } from "react";
import { Links, Meta, Outlet, Scripts, ScrollRestoration } from "react-router";
import { ThemeProvider } from "@/components/theme-provider";
// import { type AppSettings, GLOBAL_EVENTS } from "./lib/types";
import { type AppSettings, GLOBAL_EVENTS } from "@/lib/types";

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Tauri App</title>
        <Meta />
        <Links />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function Root() {
  const [isDarkMode, setIsDarkMode] = useState(true);
  listen<AppSettings>(
    GLOBAL_EVENTS.settingsUpdated,
    ({ payload: appSettings }) => {
      console.log(appSettings);
      setIsDarkMode(appSettings.isDarkmode);
    },
  );
  return (
    <ThemeProvider defaultTheme={isDarkMode ? "dark" : "light"}>
      <Outlet />
    </ThemeProvider>
  );
}
