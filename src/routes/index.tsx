import { useRoutes, Navigate, Outlet } from "react-router-dom";

import InboxIcon from "@mui/icons-material/MoveToInbox";
import MailIcon from "@mui/icons-material/Mail";

import { MainLayout, type DrawerItem } from "@/components/Layout";

import { Dashboard } from "@/features/misc";
import { XRoutes } from "@/features/settings";

const App = (): JSX.Element => {
  const drawerItems = [
    { name: "Dashboard", to: ".", icon: InboxIcon },
    { name: "Password", to: "./x", icon: MailIcon },
  ] as DrawerItem[];

  return (
    <MainLayout drawerItems={drawerItems}>
      <Outlet />
    </MainLayout>
  );
};

export const AppRoutes = (): JSX.Element => {
  const element = useRoutes([
    { path: "/settings", ...XRoutes },
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
