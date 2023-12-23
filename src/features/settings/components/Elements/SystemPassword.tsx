import { useState } from "react";

import {
  FormControl,
  TextField,
  Grid,
  Button,
  Typography,
} from "@mui/material";

import { Group } from "../Layout";

// import { formatDate } from '@/utils/format';
// import { Button } from "@/components/Elements";
import { submitSettings } from "../../api/submitSettings";

export const SystemPassword = (): JSX.Element => {
  const [data, setData] = useState<string>("");

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
            <Button
              variant="contained"
              onClick={(): void => {
                void (async (): Promise<void> => {
                  try {
                    const result = await submitSettings();
                    setData(result.body);
                  } catch (e) {
                    // console.error("Error:", e);
                    setData(e as string);
                  }
                })();
              }}
            >
              Update
            </Button>
          </Grid>
          <Typography>{data}</Typography>
        </Grid>
      </FormControl>
    </Group>
  );
};
