import { useRoutes, Navigate, Outlet } from "react-router-dom";

import { Dashboard } from "@/features/misc";

import { AppLayout } from "@/components/Layout";

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
        { path: "/x", element: <XRoutes /> },
        { path: "/", element: <Dashboard /> },
        { path: "*", element: <Navigate to="." /> },
      ],
    },
  ]);

  return <>{element}</>;
};
