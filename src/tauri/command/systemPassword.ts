import { invoke } from "@tauri-apps/api/tauri";

export const validateSystemCurrentPassword = async (currentPassword: string): Promise<boolean> => {
  return await invoke("validate_system_current_password", { currentPassword });;
};

export const updateSystemPassword = async (
  currentPassword: string,
  newPassword: string,
  confirmPassword: string
): Promise<boolean> => {
  return await invoke("update_system_password", { currentPassword, newPassword, confirmPassword });
};
