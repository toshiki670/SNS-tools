import { useState } from "react";
import { useTranslation } from "react-i18next";
import {
  FormControl,
  Grid,
  Select,
  MenuItem,
  type SelectChangeEvent,
} from "@mui/material";

import { Group } from "../Layout";

interface LanguageItem {
  value: string;
  text: string;
}

export const Language = (): JSX.Element => {
  const { i18n } = useTranslation();

  const [language, setLanguage] = useState<string>(i18n.language);

  const langageItems = [
    { value: "EnUS", text: "English" },
    { value: "JaJP", text: "日本語" },
  ] as LanguageItem[];

  const handleChange = (event: SelectChangeEvent): void => {
    setLanguage(event.target.value);
    void i18n.changeLanguage(event.target.value);
  };

  return (
    <Group title={"language"}>
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
