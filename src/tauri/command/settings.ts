import { type PaletteMode } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";

export interface Data {
  body: string;
}

export const submitSettings = async (): Promise<Data> => {
  return await invoke("submit_settings");
};

export interface Settings {
  general?: {
    language?: string;
    store_path?: string;
  };
  appearance?: {
    theme?: PaletteMode;
  };
  setting_version?: number;
}

export const getSettings = async (): Promise<Settings> => {
  return await invoke("get_settings");
};

export const updateSettings = async (settings: Settings): Promise<boolean> => {
  return await invoke("update_settings", { settings });
};
