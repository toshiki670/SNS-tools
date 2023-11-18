import { Navigate, Route, Routes } from "react-router-dom";

import { ListBlocker } from "./ListBlocker";

export const XRoutes = (): JSX.Element => {
  return (
    <Routes>
      <Route path="list_blocker" element={<ListBlocker />} />
      <Route path="*" element={<Navigate to="." />} />
    </Routes>
  );
};
