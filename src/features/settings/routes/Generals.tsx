// import { useState } from "react";

// import {
//   FormControl,
//   TextField,
//   Container,
//   Grid,
//   Box,
//   Typography,
// } from "@mui/material";

// import { Spinner } from '@/components/Elements'
import { useTranslation } from "react-i18next";

import { ContentLayout } from "@/components/Layout";
import { SystemPassword, Language } from "../components/Elements";

// import { formatDate } from '@/utils/format';
// import { Button } from "@/components/Elements";

// import { invoke } from "@tauri-apps/api/tauri";

export const Generals = (): JSX.Element => {
  const { t } = useTranslation("settings");

  return (
    <ContentLayout title={t("generals")}>
      <SystemPassword />
      <Language />
    </ContentLayout>
  );
};
