import { useTranslation } from "react-i18next";
import { Typography } from "@mui/material";

import logo from "@/assets/react.svg";

export const Logo = (): JSX.Element => {
  const { t } = useTranslation();
  return (
    <div className="flex items-center">
      <img className="h-8 w-auto" src={logo} alt="Workflow" />
      <Typography>{t("appName")}</Typography>
    </div>
  );
};
