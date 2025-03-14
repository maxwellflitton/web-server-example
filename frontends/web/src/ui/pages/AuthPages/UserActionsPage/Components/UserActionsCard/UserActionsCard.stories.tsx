import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { UserActionsCard } from "./UserActionsCard";

const meta: Meta<typeof UserActionsCard> = {
  title: "PageComponents/UserActions/UserActionsCard",
  component: UserActionsCard,
  parameters: {
    layout: "centered",
  },
  // Provide defaults for all new props
  args: {
    refetchingUser: false,
    currentRoles: ["Worker"],
    initialBlockedValue: false,
    showResendConfirmation: true,
    onBlockToggle: action("onBlockToggle"),
    blockUserError: "",
    blockUserFailure: false,
    onRolesUpdate: action("onRolesUpdate"),
    updateRolesError: "",
    updateRolesFailure: false,
    onResendConfirmation: action("onResendConfirmation"),
    resendConfirmationEmailError: "",
    resendConfirmationEmailFailure: false,
    resendConfirmationEmailSuccess: false,
    onDeleteUser: action("onDeleteUser"),
    deleteUserError: "",
    deleteUserFailure: false,
  },
  decorators: [
    (Story) => (
      <div style={{ width: "80%", margin: "5% auto" }}>
        <Story />
      </div>
    ),
  ],
};

export default meta;
type Story = StoryObj<typeof UserActionsCard>;

// 1) Default Story
export const Default: Story = {};

// 2) All Roles Selected
export const AllRolesSelected: Story = {
  args: {
    currentRoles: ["Worker", "Admin", "Super Admin"],
  },
};

// 3) No Roles Selected
export const NoRolesSelected: Story = {
  args: {
    currentRoles: [],
  },
};

// 4) Blocked User
export const BlockedUser: Story = {
  args: {
    initialBlockedValue: true,
  },
};

// 5) Unblocked User
export const UnblockedUser: Story = {
  args: {
    initialBlockedValue: false,
  },
};

// 6) Hide Resend Confirmation
export const NoResendConfirmation: Story = {
  args: {
    showResendConfirmation: false,
  },
};

// 7) Show Resend Confirmation
export const WithResendConfirmation: Story = {
  args: {
    showResendConfirmation: true,
  },
};

// 8) All Actions Triggered
export const AllActionsTriggered: Story = {
  args: {
    currentRoles: ["Worker", "Admin"],
    initialBlockedValue: true,
    showResendConfirmation: true,
    onBlockToggle: action("User blocked/unblocked"),
    onRolesUpdate: action("Roles updated"),
    onDeleteUser: action("User deleted"),
    onResendConfirmation: action("Resend confirmation email"),
  },
};

// Below are optional stories to showcase error or success states:

// 9) Block User Error
export const WithBlockUserError: Story = {
  args: {
    initialBlockedValue: true,
    blockUserError: "Error blocking user. Test scenario.",
    blockUserFailure: true,
  },
};

// 10) Refetching Spinner
export const WithRefetchingSpinner: Story = {
  args: {
    refetchingUser: true,
  },
};

// 11) Resend Email Success
export const WithResendEmailSuccess: Story = {
  args: {
    resendConfirmationEmailSuccess: true,
    showResendConfirmation: true,
  },
};
