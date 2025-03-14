import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { BlockUserForm } from "./BlockUserForm";

const meta: Meta<typeof BlockUserForm> = {
  title: "PageComponents/UserActions/BlockUserForm",
  component: BlockUserForm,
  parameters: {
    layout: "centered",
  },
  // Provide default values for new props here
  args: {
    initialBlockedValue: false,
    blockUserError: "",
    blockUserFailure: false,
    onSubmit: action("onSubmit"),
  },
  decorators: [
    (Story) => (
      <div style={{ width: "60%", margin: "5% auto" }}>
        <Story />
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof BlockUserForm>;

export const Default: Story = {};

export const Blocked: Story = {
  args: {
    initialBlockedValue: true,
  },
};

export const Unblocked: Story = {
  args: {
    initialBlockedValue: false,
  },
};

export const WithSubmitAction: Story = {
  args: {
    initialBlockedValue: true,
    onSubmit: action("Form submitted with blocked: true"),
  },
};

// If you'd like a story to show the error alert:
export const WithErrorAlert: Story = {
  args: {
    initialBlockedValue: true,
    blockUserError: "Blocking user failed. Please try again.",
    blockUserFailure: true,
  },
};
