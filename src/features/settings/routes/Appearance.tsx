import { useTranslation } from "react-i18next";

import { ContentLayout } from "@/components/Layout";
import { Theme } from "../components/Elements";

export const Appearance = (): JSX.Element => {
  const { t } = useTranslation("settings");

  return (
    <ContentLayout title={t("appearance")}>
      <Theme />
    </ContentLayout>
  );
};
