import { Navigate } from "react-router-dom";

import "./i18n/configs";
import { List } from "./pages/List";
import { Show } from "./pages/Show";

export const ItemsRoutes = {
  children: [
    { path: "", element: <List /> },
    { path: ":id", element: <Show /> },
    { path: "*", element: <Navigate to="." /> },
  ],
};
