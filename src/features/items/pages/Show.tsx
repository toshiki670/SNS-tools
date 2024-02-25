import { useParams } from "react-router-dom";
import { useTranslation } from "react-i18next";

import { ContentLayout } from "@/components/Layout";
import { Typography } from "@mui/material";

export const Show = (): JSX.Element => {
  const { t } = useTranslation("items", {
    keyPrefix: "pages.Show",
  });
  const { id } = useParams();

  console.log(id);

  return (
    <ContentLayout title={t("title")}>
      <Typography>{id}</Typography>
    </ContentLayout>
  );
};
