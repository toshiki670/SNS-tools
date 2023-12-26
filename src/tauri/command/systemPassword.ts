import { invoke } from "@tauri-apps/api/tauri";

export const validateSystemCurrentPassword = async (current: string): Promise<boolean> => {
  return await invoke("validate_system_current_password", { current });;
};

export const updateSystemPassword = async (
  current: string,
  password: string,
  confirm: string
): Promise<string> => {
  return await invoke("update_system_password", { current, password, confirm });
};
