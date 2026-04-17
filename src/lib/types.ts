export type AppSettings = {
  is_darkmode: boolean;
  project_folder?: string;
}

export const GLOBAL_EVENTS = {
  settingsUpdated: "settings-updated"
}

export const COMMANDS = {
  GET_SETTINGS: "get_settings",
}
