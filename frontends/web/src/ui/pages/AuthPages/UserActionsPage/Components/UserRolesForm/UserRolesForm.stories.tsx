import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { UserRolesForm } from "./UserRolesForm";

const meta: Meta<typeof UserRolesForm> = {
  title: "PageComponents/UserActions/UserRolesForm",
  component: UserRolesForm,
  parameters: {
    layout: "centered",
  },
  // Default args for all stories
  args: {
    currentRoles: [],
    updateRolesError: "",
    updateRolesFailure: false,
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
type Story = StoryObj<typeof UserRolesForm>;

export const Default: Story = {};

export const AllRolesAssigned: Story = {
  args: {
    currentRoles: ["Worker", "Admin", "Super Admin"],
  },
};

export const OnlyAdminRole: Story = {
  args: {
    currentRoles: ["Admin"],
  },
};

export const NoRolesAssigned: Story = {
  args: {
    currentRoles: [],
  },
};

export const SubmitAction: Story = {
  args: {
    currentRoles: ["Worker"],
    onSubmit: action("Form submitted with roles"),
  },
};

// Optional: Story to show an error scenario
export const WithErrorAlert: Story = {
  args: {
    currentRoles: ["Admin"],
    updateRolesError: "Failed to update roles. Please try again.",
    updateRolesFailure: true,
  },
};
