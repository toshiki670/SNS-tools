import {
  ChatBubbleBottomCenterTextIcon,
  HomeIcon,
} from "@heroicons/react/24/outline";
import clsx from "clsx";
import type * as React from "react";
import { NavLink } from "react-router-dom";

interface SideNavigationItem {
  name: string;
  to: string;
  icon: (props: React.SVGProps<SVGSVGElement>) => JSX.Element;
}

export const SideNavigation = (): JSX.Element => {
  const navigationItems = [
    { name: "Dashboard", to: ".", icon: HomeIcon },
    { name: "X(Twitter)", to: "./x", icon: ChatBubbleBottomCenterTextIcon },
  ].filter(Boolean) as SideNavigationItem[];

  return (
    <>
      {navigationItems.map((item, index) => (
        <NavLink
          end={index === 0}
          key={item.name}
          to={item.to}
          className={({ isActive }) =>
            clsx([
              isActive ? "bg-gray-900 text-white" : "",
              "text-gray-300 hover:bg-gray-700 hover:text-white",
              "group flex items-center px-2 py-2 text-base font-medium rounded-md",
            ])
          }
        >
          <item.icon
            className={clsx(
              "text-gray-400 group-hover:text-gray-300",
              "mr-4 flex-shrink-0 h-6 w-6"
            )}
            aria-hidden="true"
          />
          {item.name}
        </NavLink>
      ))}
    </>
  );
};
