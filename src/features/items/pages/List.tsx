import { useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

import { ContentLayout } from "@/components/Layout";
import { Typography } from "@mui/material";

export const List = (): JSX.Element => {
  const { t } = useTranslation("items", {
    keyPrefix: "pages.List",
  });

  return (
    <ContentLayout title={t("title")}>
      <Typography component={NavLink} to={"1"}>
        aaaa
      </Typography>
    </ContentLayout>
  );
};
