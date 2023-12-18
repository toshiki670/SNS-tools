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
import { ContentLayout } from "@/components/Layout";
import { SystemPassword } from "../components/Elements";

// import { formatDate } from '@/utils/format';
// import { Button } from "@/components/Elements";

// import { invoke } from "@tauri-apps/api/tauri";

export const Generals = (): JSX.Element => {


  return (
    <ContentLayout title={"Generals"}>
      <SystemPassword />
    </ContentLayout>
  );
};
