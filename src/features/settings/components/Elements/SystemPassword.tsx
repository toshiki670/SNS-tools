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
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
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
      currentPassword: passwordValidation.refine(validateSystemCurrentPassword, {
        message: t("error.wrongCurrentPassword"),
      }),
      newPassword: passwordValidation,
      confirmPassword: passwordValidation,
    })
    .superRefine(({ currentPassword, newPassword, confirmPassword }, ctx) => {
      // Correlation Validation
      if (newPassword !== confirmPassword) {
        ctx.addIssue({
          path: ["confirmPassword"],
          code: "custom",
          message: t("error.notSame", { label: t("newPassword") }),
        });
      }
      if (currentPassword === newPassword) {
        ctx.addIssue({
          path: ["newPassword"],
          code: "custom",
          message: t("error.same", { label: t("currentPassword") }),
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

  const [submitResult, setSubmitResult] = useState<string>("");

  const onSubmit: SubmitHandler<SystemPasswordFormInput> = async (
    data: SystemPasswordFormInput
  ) => {
    const result = await updateSystemPassword(
      data.currentPassword,
      data.newPassword,
      data.confirmPassword
    );
    const message = result ? t("success"): t("Failed");
    setSubmitResult(message);
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
              {...register("currentPassword")}
              error={errors.currentPassword !== undefined}
              helperText={errors.currentPassword?.message}
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label={t("newPassword")}
              type="password"
              {...register("newPassword")}
              error={errors.newPassword !== undefined}
              helperText={errors.newPassword?.message}
            />
          </Grid>
          <Grid item xs={8}>
            <TextField
              required
              fullWidth
              label={t("confirmPassword")}
              type="password"
              {...register("confirmPassword")}
              error={errors.confirmPassword !== undefined}
              helperText={errors.confirmPassword?.message}
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
