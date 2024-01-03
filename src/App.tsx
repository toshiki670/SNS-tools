import { Providers } from "@/providers";
import { AppRoutes } from "@/routes";

export const App = (): JSX.Element => {
  return (
    <Providers>
      <AppRoutes />
    </Providers>
  );
};
