import { Navigate, Outlet } from "react-router-dom";
import { useTranslation } from "react-i18next";
import SettingsIcon from "@mui/icons-material/Settings";
import ColorLensIcon from "@mui/icons-material/ColorLens";

import { MainLayout, type DrawerItem } from "@/components/Layout";

import "../i18n/configs";
import { Generals } from "./Generals";
import { Appearance } from "./Appearance";

const Settings = (): JSX.Element => {
  const { t } = useTranslation("settings");

  const drawerItems = [
    { name: t("generals"), to: "./generals", icon: SettingsIcon },
    { name: t("appearance"), to: "./appearance", icon: ColorLensIcon },
  ] as DrawerItem[];

  return (
    <MainLayout drawerItems={drawerItems}>
      <Outlet />
    </MainLayout>
  );
};

export const SettingsRoutes = {
  element: <Settings />,
  children: [
    { path: "generals", element: <Generals /> },
    { path: "appearance", element: <Appearance /> },
    { path: "*", element: <Navigate to="." /> },
  ],
};
