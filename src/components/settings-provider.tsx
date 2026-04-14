import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  createContext,
  type PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react";
import { type AppSettings, COMMANDS, GLOBAL_EVENTS } from "@/lib/types";

// MAYBE: somehow sync defaults, but should always take from settings.json
const initialState: AppSettings = {
  isDarkmode: true,
};
const AppSettingsContext = createContext<AppSettings>(initialState);

export function AppSettingsProvider({ children, ...props }: PropsWithChildren) {
  const [appSettings, setAppSettings] = useState<AppSettings>(initialState);
  useEffect(() => {
    // Call to set it up once
    invoke<AppSettings>(COMMANDS.GET_SETTINGS).then((updatedAppSettings) =>
      setAppSettings(updatedAppSettings),
    );
    // Listen for any subsequent updates
    listen<AppSettings>(
      GLOBAL_EVENTS.settingsUpdated,
      ({ payload: updatedAppSettings }) => {
        setAppSettings(updatedAppSettings);
      },
    );
  }, []);

  return (
    <AppSettingsContext.Provider {...props} value={appSettings}>
      {children}
    </AppSettingsContext.Provider>
  );
}

export const useAppSettings = () => {
  const context = useContext(AppSettingsContext);

  if (context === undefined)
    throw new Error("useTheme must be used within a AppSettingsProvider");

  return context;
};
