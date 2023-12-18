// import { useState } from "react";

import {
  FormControl,
  TextField,
  Grid,
  Button,
} from "@mui/material";

import { Group } from "../Layout";

// import { formatDate } from '@/utils/format';
// import { Button } from "@/components/Elements";

// import { invoke } from "@tauri-apps/api/tauri";

export const SystemPassword = (): JSX.Element => {
  return (
    <Group title={"System Password"}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              id="outlined-password-input"
              label="Current Password"
              type="password"
              autoComplete="current-password"
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              id="outlined-password-input"
              label="New Password"
              type="password"
              autoComplete="current-password"
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              id="outlined-password-input"
              label="Confirm Password"
              type="password"
              autoComplete="current-password"
            />
          </Grid>
          <Grid item xs={1}>
            <Button variant="contained">Update</Button>
          </Grid>
        </Grid>
      </FormControl>
    </Group>
  );
};
