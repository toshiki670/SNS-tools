/* eslint-disable @typescript-eslint/no-misused-promises */
// Form reference
// https://dev.classmethod.jp/articles/mui-v5-rhf-v7/

import { type SubmitHandler, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import z from "@/configs/zod";

import { FormControl, TextField, Grid, Button } from "@mui/material";
import { useTranslation } from "react-i18next";

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
  const { t } = useTranslation("settings", {
    keyPrefix: "components.SystemPassword",
  });

  const passwordValidation = z
    .string()
    .min(8)
    .regex(
      /^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*#?&])[A-Za-z\d@$!%*#?&].*$/,
      t("error.includeLetters")
    );

  const schema = z
    .object({
      current: passwordValidation.refine(validateSystemCurrentPassword, {
        message: t("error.wrongCurrentPassword"),
      }),
      password: passwordValidation,
      confirm: passwordValidation,
    })
    .superRefine(({ current, password, confirm }, ctx) => {
      // Correlation Validation
      if (password !== confirm) {
        ctx.addIssue({
          path: ["confirm"],
          code: "custom",
          message: t("error.notSame", { label: t("newPassword")}),
        });
      }
      if (current === password) {
        ctx.addIssue({
          path: ["password"],
          code: "custom",
          message: t("error.same", { label: t("currentPassword")}),
        });
      }
    });

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<SystemPasswordFormInput>({
    mode: "onChange",
    resolver: zodResolver(schema),
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
    <Group title={t("title")}>
      <FormControl>
        <Grid container spacing={2}>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label={t("currentPassword")}
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
              label={t("newPassword")}
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
              label={t("confirmPassword")}
              type="password"
              autoComplete="confirm-password"
              {...register("confirm")}
              error={errors.confirm !== undefined}
              helperText={errors.confirm?.message}
            />
          </Grid>
          <Grid item xs={1}>
            <Button variant="contained" onClick={handleSubmit(onSubmit)}>
              {t("submit")}
            </Button>
          </Grid>
          {/* <Typography>{data}</Typography> */}
        </Grid>
      </FormControl>
    </Group>
  );
};
