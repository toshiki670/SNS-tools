import { type Meta, type StoryObj } from "@storybook/react";

import { Link } from "./Link";

const meta: Meta<typeof Link> = {
  component: Link,
  parameters: {
    controls: { expanded: true },
  },
};

export default meta;
type Story = StoryObj<typeof Link>;


export const Template: Story = {
  args: {
    children: <>Hello</>,
    to: "/",
  },
};