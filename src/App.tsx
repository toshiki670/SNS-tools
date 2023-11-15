import { AppProvider } from "@/providers/app";
import { AppRoutes } from "@/routes";

export const App = (): JSX.Element => {
  return (
    <AppProvider>
      <AppRoutes />
    </AppProvider>
  );
};
