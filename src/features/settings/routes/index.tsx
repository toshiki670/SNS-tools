import { Navigate, Outlet } from "react-router-dom";

import SettingsIcon from "@mui/icons-material/Settings";
import ColorLensIcon from "@mui/icons-material/ColorLens";

import { MainLayout, type DrawerItem } from "@/components/Layout";
import { Generals } from "./Generals";

const Settings = (): JSX.Element => {
  const drawerItems = [
    { name: "Generals", to: "./generals", icon: SettingsIcon },
    { name: "Appearance", to: "./appearance", icon: ColorLensIcon },
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
    { path: "appearance", element: <Navigate to="." /> },
    { path: "*", element: <Navigate to="." /> },
  ],
};
