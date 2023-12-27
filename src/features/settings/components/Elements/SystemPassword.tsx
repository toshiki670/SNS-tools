/* eslint-disable @typescript-eslint/no-misused-promises */
// Form reference
// https://dev.classmethod.jp/articles/mui-v5-rhf-v7/
// https://github.com/jquense/yup

import { type SubmitHandler, useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";
import { FormControl, TextField, Grid, Button } from "@mui/material";

import { Group } from "../Layout";

import {
  validateSystemCurrentPassword,
  updateSystemPassword,
} from "@/tauri/command";

interface SystemPasswordFormInput {
  current: string;
  password: string;
  confirm: string;
}

export const SystemPassword = (): JSX.Element => {
  const passwordValidation = yup
    .string()
    .required("required")
    .min(8, "less")
    .matches(
      /^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*#?&])[A-Za-z\d@$!%*#?&].*$/,
      "week password"
    );

  const schema = yup.object({
    current: yup
      .string()
      .concat(passwordValidation)
      .test("match_current", "not match", async (value, _) => {
        return await validateSystemCurrentPassword(value);
      }),
    password: yup.string().concat(passwordValidation),
    confirm: yup
      .string()
      .concat(passwordValidation)
      .oneOf([yup.ref("password")], "Passwords must match"),
  });

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<SystemPasswordFormInput>({
    mode: "onChange",
    resolver: yupResolver(schema),
  });

  const onSubmit: SubmitHandler<SystemPasswordFormInput> = async (
    data: SystemPasswordFormInput
  ) => {
    const result = await updateSystemPassword(
      data.current,
      data.password,
      data.confirm
    );
    console.log(result);
  };

  return (
    <Group title={"System Password"}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label="Current Password"
              type="password"
              autoComplete="current-password"
              {...register("current")}
              error={errors.current !== undefined}
              helperText={errors.current?.message}
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label="New Password"
              type="password"
              autoComplete="new-password"
              {...register("password")}
              error={errors.password !== undefined}
              helperText={errors.password?.message}
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label="Confirm Password"
              type="password"
              autoComplete="confirm-password"
              {...register("confirm")}
              error={errors.confirm !== undefined}
              helperText={errors.confirm?.message}
            />
          </Grid>
          <Grid item xs={1}>
            <Button variant="contained" onClick={handleSubmit(onSubmit)}>
              Update
            </Button>
          </Grid>
          {/* <Typography>{data}</Typography> */}
        </Grid>
      </FormControl>
    </Group>
  );
};
