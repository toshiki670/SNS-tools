import { useRoutes, Navigate, Outlet } from "react-router-dom";
import { useTranslation } from "react-i18next";
import InboxIcon from "@mui/icons-material/MoveToInbox";
import MailIcon from "@mui/icons-material/Mail";

import { MainLayout, type DrawerItem } from "@/components/Layout";

import { Dashboard } from "@/features/misc";
import { SettingsRoutes } from "@/features/settings";

const App = (): JSX.Element => {
  const { t } = useTranslation();

  const drawerItems = [
    { name: "Dashboard", to: ".", icon: InboxIcon },
    { name: t("features.items.all"), to: "./items", icon: MailIcon },
  ] as DrawerItem[];

  return (
    <MainLayout drawerItems={drawerItems}>
      <Outlet />
    </MainLayout>
  );
};

export const AppRoutes = (): JSX.Element => {
  const element = useRoutes([
    { path: "/settings", ...SettingsRoutes },
    {
      path: "/",
      element: <App />,
      children: [
        { path: "/", ...Dashboard },
        { path: "*", element: <Navigate to="." /> },
      ],
    },
  ]);

  return <>{element}</>;
};
