import { Link } from "react-router-dom";

import logo from "@/assets/react.svg";

export const Logo = (): JSX.Element => {
  return (
    <Link className="flex items-center text-white" to=".">
      <img className="h-8 w-auto" src={logo} alt="Workflow" />
      <span className="text-xl text-white font-semibold">SNS Tools</span>
    </Link>
  );
};
