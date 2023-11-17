import { type Meta, type StoryObj } from "@storybook/react";

import { Link } from "./Link";

export default {
  component: Link,
  parameters: {
    controls: { expanded: true },
  },
} satisfies Meta<typeof Link>;

type Story = StoryObj<typeof Link>;

export const Template: Story = {
  args: {
    children: <>Hello</>,
    to: "/",
  },
};
