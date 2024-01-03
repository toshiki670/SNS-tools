import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter } from "react-router-dom";

import { CustomThemeProvider } from "./CustomThemeProvider";

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers = ({ children }: ProvidersProps): JSX.Element => {
  return (
    <CustomThemeProvider>
      <HelmetProvider>
        <BrowserRouter>{children}</BrowserRouter>
      </HelmetProvider>
    </CustomThemeProvider>
  );
};
