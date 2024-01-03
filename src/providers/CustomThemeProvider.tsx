import { useEffect, useState } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { type PaletteMode } from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import { PaletteModeCtx } from "@/ctx/PaletteModeCtx";

import { getSettings } from "@/tauri/command";

interface CustomThemeProviderProps {
  children: React.ReactNode;
}

export const CustomThemeProvider = ({
  children,
}: CustomThemeProviderProps): JSX.Element => {
  const [mode, setMode] = useState<PaletteMode>("dark");

  const theme = createTheme({
    palette: {
      mode,
    },
  });

  useEffect(() => {
    void (async () => {
      try {
        const theme = (await getSettings()).appearance?.theme;
        if (theme !== undefined) {
          setMode(theme);
        }
      } catch (error) {
        console.error(error);
      }
    })();
  }, []);

  return (
    <PaletteModeCtx.Provider value={{ mode, setMode }}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </PaletteModeCtx.Provider>
  );
};
