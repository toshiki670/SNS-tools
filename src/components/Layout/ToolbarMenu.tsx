import * as React from "react";
import { useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

import { IconButton, Menu, MenuItem } from "@mui/material";
import SettingsIcon from "@mui/icons-material/Settings";

export const ToolbarMenu = (): JSX.Element => {
  const { t } = useTranslation();
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const handleMenu = (event: React.MouseEvent<HTMLElement>): void => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = (): void => {
    setAnchorEl(null);
  };

  return (
    <>
      <IconButton
        size="large"
        aria-label="account of current user"
        aria-controls="menu-appbar"
        aria-haspopup="true"
        onClick={handleMenu}
        color="inherit"
      >
        <SettingsIcon />
      </IconButton>
      <Menu
        id="menu-appbar"
        anchorEl={anchorEl}
        anchorOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        keepMounted
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        open={Boolean(anchorEl)}
        onClose={handleClose}
      >
        <MenuItem onClick={handleClose}>{t("features.lock")}</MenuItem>
        <MenuItem
          component={NavLink}
          to={"/settings/generals"}
          onClick={handleClose}
        >
          {t("features.settings")}
        </MenuItem>
      </Menu>
    </>
  );
};
