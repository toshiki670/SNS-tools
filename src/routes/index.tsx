import { useRoutes, Navigate, Outlet } from "react-router-dom";

import { AppLayout } from "@/components/Layout";

import { Dashboard } from "@/features/misc";
import { XRoutes } from "@/features/x";

const App = (): JSX.Element => {
  return (
    <AppLayout>
      <Outlet />
    </AppLayout>
  );
};

export const AppRoutes = (): JSX.Element => {
  const element = useRoutes([
    {
      path: "/",
      element: <App />,
      children: [
        { path: "/x", ...XRoutes },
        { path: "/", ...Dashboard },
        { path: "*", element: <Navigate to="." /> },
      ],
    },
  ]);

  return <>{element}</>;
};
