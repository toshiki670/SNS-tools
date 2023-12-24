import { invoke } from "@tauri-apps/api/tauri";

export interface Data {
  body: string;
}

export const submitSettings = async (): Promise<Data> => {
  return await invoke("submit_settings");
};
