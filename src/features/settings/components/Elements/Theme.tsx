import { useContext } from "react";
import { useTranslation } from "react-i18next";
import {
  FormControl,
  Grid,
  Select,
  MenuItem,
  type SelectChangeEvent,
  type PaletteMode,
} from "@mui/material";

import { PaletteModeCtx } from "@/ctx/PaletteModeCtx";
import { updateSettings } from "@/tauri/command";

import { Group } from "../Layout";

interface ThemeItem {
  value: PaletteMode;
  text: string;
}

export const Theme = (): JSX.Element => {
  const { t } = useTranslation("settings", {
    keyPrefix: "components.Theme",
  });

  const context = useContext(PaletteModeCtx);
  const mode = context?.mode;
  const setMode = context?.setMode;

  const themeItems = [
    { value: "light", text: t("light") },
    { value: "dark", text: t("dark") },
  ] as ThemeItem[];

  const handleChange = (event: SelectChangeEvent): void => {
    if (setMode === undefined) {
      return;
    }
    const changedValue = event.target.value as PaletteMode;
    void updateSettings({
      appearance: { theme: changedValue },
    });
    setMode(changedValue);
  };

  return (
    <Group title={t("title")}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <Select
              label="Theme"
              value={mode ?? "dark"}
              onChange={handleChange}
            >
              {themeItems.map((item) => (
                <MenuItem key={item.value} value={item.value}>
                  {item.text}
                </MenuItem>
              ))}
            </Select>
          </Grid>
        </Grid>
      </FormControl>
    </Group>
  );
};
