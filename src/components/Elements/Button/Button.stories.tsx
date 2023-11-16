import { type Meta, type StoryObj } from "@storybook/react";

import { Button } from "./Button";

export default {
  title: "Components/Elements/Button",
  component: Button,
  parameters: {
    controls: { expanded: true },
  },
} satisfies Meta<typeof Button>;

type Story = StoryObj<typeof Button>;

export const Primary: Story = {
  args: {
    children: "Primary Button",
    variant: "primary",
  },
};

export const Inverse: Story = {
  args: {
    children: "Inverse Button",
    variant: "inverse",
  },
};

export const Danger: Story = {
  args: {
    children: "Danger Button",
    variant: "danger",
  },
};
