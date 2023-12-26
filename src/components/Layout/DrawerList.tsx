import type * as React from "react";

import { NavLink } from "react-router-dom";

import {
  Divider,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
} from "@mui/material";

import { Logo } from "./Logo";
interface DrawerListProps {
  items: DrawerItem[];
}

export interface DrawerItem {
  name: string;
  to: string;
  icon: (props: React.SVGProps<SVGSVGElement>) => JSX.Element;
}

export const DrawerList = ({ items }: DrawerListProps): JSX.Element => {
  return (
    <>
      <Toolbar component={NavLink} to={"/"}>
        <Logo />
      </Toolbar>
      <Divider />
      <List>
        {items.map((item) => (
          <ListItem
            key={item.name}
            disablePadding
            component={NavLink}
            to={item.to}
          >
            <ListItemButton>
              <ListItemIcon>
                <item.icon />
              </ListItemIcon>
              <ListItemText primary={item.name} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </>
  );
};
