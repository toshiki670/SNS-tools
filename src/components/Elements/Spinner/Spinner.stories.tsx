import { type Meta, type Story } from "@storybook/react";

import { Spinner, type SpinnerProps } from "./Spinner";

const meta: Meta = {
  component: Spinner,
  parameters: {
    controls: { expanded: true },
  },
};

export default meta;

const Template: Story<SpinnerProps> = (props) => <Spinner {...props} />;

export const Default = Template.bind({});
Default.args = {};
