import { Links, Meta, Outlet, Scripts, ScrollRestoration } from "react-router";
import { ThemeProvider } from "@/components/theme-provider";
import { AppSettingsProvider, useAppSettings } from "./components/AppSettingsProvider";

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
  return (
    <AppSettingsProvider>
      <ThemeProvider>
        <Outlet />
      </ThemeProvider>
    </AppSettingsProvider>
  );
}
