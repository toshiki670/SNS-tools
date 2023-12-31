import { useTranslation } from "react-i18next";

import logo from "@/assets/react.svg";

export const Logo = (): JSX.Element => {
  const { t } = useTranslation();
  return (
    <div className="flex items-center text-white">
      <img className="h-8 w-auto" src={logo} alt="Workflow" />
      <span className="text-xl text-black font-semibold">{t("appName")}</span>
    </div>
  );
};
