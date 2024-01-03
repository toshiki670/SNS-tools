import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter } from "react-router-dom";

import { CustomThemeProvider } from "./CustomThemeProvider";

interface AppProviderProps {
  children: React.ReactNode;
}

export const AppProvider = ({ children }: AppProviderProps): JSX.Element => {
  return (
    <CustomThemeProvider>
      <HelmetProvider>
        <BrowserRouter>{children}</BrowserRouter>
      </HelmetProvider>
    </CustomThemeProvider>
  );
};
