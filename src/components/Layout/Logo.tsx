import logo from "@/assets/react.svg";

export const Logo = (): JSX.Element => {
  return (
    <div className="flex items-center text-white">
      <img className="h-8 w-auto" src={logo} alt="Workflow" />
      <span className="text-xl text-black font-semibold">Passwords</span>
    </div>
  );
};
