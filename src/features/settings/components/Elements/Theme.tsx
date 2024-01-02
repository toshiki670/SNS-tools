import { useState } from "react";
import { useTranslation } from "react-i18next";
import {
  FormControl,
  Grid,
  Select,
  MenuItem,
  type SelectChangeEvent,
} from "@mui/material";

import { updateSettings } from "@/tauri/command";

import { Group } from "../Layout";

interface ThemeItem {
  value: string;
  text: string;
}

export const Theme = (): JSX.Element => {
  const { t } = useTranslation("settings", {
    keyPrefix: "components.Theme",
  });

  const [theme, setTheme] = useState<string>("light");

  const themeItems = [
    { value: "light", text: t("light") },
    { value: "dark", text: t("dark") },
  ] as ThemeItem[];

  const handleChange = (event: SelectChangeEvent): void => {
    const changedValue = event.target.value;
    setTheme(changedValue);
  };

  return (
    <Group title={t("title")}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <Select label="Theme" value={theme} onChange={handleChange}>
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
