import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { ConfirmUserForm } from "./ConfirmUserForm";

const meta: Meta<typeof ConfirmUserForm> = {
  title: "PageComponents/ConfirmUser/ConfirmUserForm",
  component: ConfirmUserForm,
  parameters: {
    layout: "centered",
  },
  args: {
    // User details for a successful fetch.
    firstName: "John",
    lastName: "Doe",
    username: "johndoe",
    // No error by default.
    getUserError: "",
    getUserFailure: false,
    confirmUserError: "",
    confirmUserFailure: false,
    disabled: false,
    submitConfirmUserForm: action("submitConfirmUserForm"),
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
type Story = StoryObj<typeof ConfirmUserForm>;

export const Default: Story = {
  args: {
    // Use default args provided in meta.
  },
};

export const WithConfirmUserFailure: Story = {
  args: {
    confirmUserFailure: true,
    confirmUserError: "Unable to confirm account. Please try again later.",
  },
};

export const WithGetUserFailure: Story = {
  args: {
    getUserFailure: true,
    getUserError: "Failed to fetch user details. Please reopen the email link and try again.",
    // Disable the form when fetching the user details fails.
    disabled: true,
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};
