import { type Meta, type StoryObj } from "@storybook/react";

import { Spinner } from "./Spinner";

export default {
  component: Spinner,
  parameters: {
    controls: { expanded: true },
  },
} satisfies Meta<typeof Spinner>;

type Story = StoryObj<typeof Spinner>;

export const Template: Story = {};
