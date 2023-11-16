import { type Meta, type StoryObj } from "@storybook/react";

import { Button } from "../Button";

import { ConfirmationDialog } from "./ConfirmationDialog";

const meta: Meta<typeof ConfirmationDialog> = {
  title: "Components/Elements/ConfirmationDialog",
  component: ConfirmationDialog,
  parameters: {
    controls: { expanded: true },
  },
};

export default meta;
type Story = StoryObj<typeof ConfirmationDialog>;

export const Danger: Story = {
  args: {
    icon: "danger",
    title: "Confirmation",
    body: "Hello World",
    confirmButton: <Button className="bg-red-500">Confirm</Button>,
    triggerButton: <Button>Open</Button>,
  },
};

export const Info: Story = {
  args: {
    icon: "info",
    title: "Confirmation",
    body: "Hello World",
    confirmButton: <Button>Confirm</Button>,
    triggerButton: <Button>Open</Button>,
  },
};
