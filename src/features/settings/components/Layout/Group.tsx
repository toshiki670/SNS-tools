import type * as React from "react";

import { Box, Typography } from "@mui/material";

interface GroupProps {
  title: string;
  children: React.ReactNode;
}

export const Group = ({ title, children }: GroupProps): JSX.Element => {
  return (
    <Box
      border={1}
      borderColor="primary.dark"
      borderRadius={1}
      pt={3}
      px={2}
      pb={2}
      position="relative"
    >
      <Typography
        variant="subtitle1"
        component="div"
        position="absolute"
        top={-5}
        left={20}
        bgcolor="background.paper"
        pl={1}
        pr={1}
        ml={-1}
        mt={-1}
        zIndex={1}
      >
        {title}
      </Typography>
      {children}
    </Box>
  );
};
