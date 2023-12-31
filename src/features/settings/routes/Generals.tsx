import { useTranslation } from "react-i18next";

import { ContentLayout } from "@/components/Layout";
import { SystemPassword, Language } from "../components/Elements";

export const Generals = (): JSX.Element => {
  const { t } = useTranslation("settings");

  return (
    <ContentLayout title={t("generals")}>
      <SystemPassword />
      <Language />
    </ContentLayout>
  );
};
