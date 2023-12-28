import { invoke } from "@tauri-apps/api/tauri";

export interface Data {
  body: string;
}

export const submitSettings = async (): Promise<Data> => {
  return await invoke("submit_settings");
};

// [Log] {appearance: {theme: "Light"}, general: {language: "EnUS", store_path: "../password.json"}, setting_version: 1} (configs.ts, line 6)

export interface Settings {
  general: {
    language?: string;
    store_path?: string;
  };
  appearance?: {
    theme?: string;
  };
  setting_version?: number;
}

export const getSettings = async (): Promise<Settings> => {
  return await invoke("get_settings");
};

export const updateSettings = async (settings: Settings): Promise<string> => {
  return await invoke("update_settings", { settings });
};
