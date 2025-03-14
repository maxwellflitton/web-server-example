import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { LoginForm } from "./LoginForm";

const meta: Meta<typeof LoginForm> = {
  title: "PageComponents/Login/LoginForm",
  component: LoginForm,
  parameters: {
    layout: "centered",
  },
  args: {
    apiError: "",
    apiFailure: false,
    submitLoginForm: action("submitLoginForm"),
  },
  decorators: [
    (Story) => (
      <div style={{ width: "80%", margin: "20% auto" }}>
        <Story />
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof LoginForm>;

export const Default: Story = {
  args: {},
};

export const WithApiFailure: Story = {
  args: {
    apiFailure: true,
    apiError: "Unable to login. Please try again later.",
  },
};
