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

interface LanguageItem {
  value: string;
  text: string;
}

export const Language = (): JSX.Element => {
  const { t, i18n } = useTranslation("settings", { keyPrefix: 'components.Language' });

  const [language, setLanguage] = useState<string>(i18n.language);

  const langageItems = [
    { value: "en-US", text: "English" },
    { value: "ja-JP", text: "日本語" },
  ] as LanguageItem[];

  const handleChange = (event: SelectChangeEvent): void => {
    const changedValue = event.target.value;
    setLanguage(changedValue);
    void i18n.changeLanguage(changedValue);
    void updateSettings({ general: { language: changedValue } });
  };

  return (
    <Group title={t("title")}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <Select label="Language" value={language} onChange={handleChange}>
              {langageItems.map((item) => (
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
