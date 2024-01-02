import type React from "react";
import { useState } from "react";
import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter } from "react-router-dom";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { type PaletteMode } from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";

interface AppProviderProps {
  children: React.ReactNode;
}

export const AppProvider = ({ children }: AppProviderProps): JSX.Element => {
  const [mode, setMode] = useState<PaletteMode>("dark");
  const theme = createTheme({
    palette: {
      mode,
    },
  });

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <HelmetProvider>
        <BrowserRouter>{children}</BrowserRouter>
      </HelmetProvider>
    </ThemeProvider>
  );
};
