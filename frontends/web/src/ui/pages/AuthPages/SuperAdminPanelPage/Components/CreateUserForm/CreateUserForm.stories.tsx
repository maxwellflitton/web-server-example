// CreateUserForm.stories.tsx

import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { CreateUserForm } from "./CreateUserForm";

const meta: Meta<typeof CreateUserForm> = {
  title: "PageComponents/CreateUser/CreateUserForm",
  component: CreateUserForm,
  parameters: {
    layout: "centered",
  },
  args: {
    // Default prop values for all stories.
    apiError: "",
    apiFailure: false,
    isSubmittingCreateUserForm: false,
    submitCreateUserForm: action("submitCreateUserForm"),
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
type Story = StoryObj<typeof CreateUserForm>;

export const Default: Story = {
  args: {
    // Inherits from meta.args, so nothing extra needed
  },
};

export const WithApiFailure: Story = {
  args: {
    apiFailure: true,
    apiError: "Failed to create account. Please try again later.",
  },
};

export const WithSubmittingOverlay: Story = {
  args: {
    isSubmittingCreateUserForm: true,
  },
};
