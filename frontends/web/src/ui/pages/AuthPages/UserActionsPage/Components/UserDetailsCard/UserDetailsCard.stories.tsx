import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { UserDetailsCard } from "./UserDetailsCard";


const meta: Meta<typeof UserDetailsCard> = {
  title: "PageComponents/UserActions/UserDetailsCard",
  component: UserDetailsCard,
  parameters: {
    layout: "centered",
  },
  args: {
    user: {
      username: "john.doe",
      email: "john.doe@example.com",
      firstName: "John",
      lastName: "Doe",
      userRole: ["Worker"],
      dateCreated: "2023-01-01T00:00:00.000Z",
      lastLoggedIn: "2023-06-01T00:00:00.000Z",
      blocked: false,
      confirmed: true,
    },
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
type Story = StoryObj<typeof UserDetailsCard>;

export const Default: Story = {};

export const MultipleRoles: Story = {
  args: {
    user: {
      username: "multi.user",
      email: "multi.user@example.com",
      firstName: "Multi",
      lastName: "Role",
      userRole: ["Worker", "Admin", "Super Admin"],
      dateCreated: "2022-12-15T00:00:00.000Z",
      lastLoggedIn: "2023-05-15T00:00:00.000Z",
      blocked: false,
      confirmed: true,
    },
  },
};

export const BlockedUser: Story = {
  args: {
    user: {
      username: "blocked.user",
      email: "blocked.user@example.com",
      firstName: "Blocked",
      lastName: "User",
      userRole: ["Admin"],
      dateCreated: "2023-02-20T00:00:00.000Z",
      lastLoggedIn: "2023-05-20T00:00:00.000Z",
      blocked: true,
      confirmed: true,
    },
  },
};

export const UnconfirmedUser: Story = {
  args: {
    user: {
      username: "unconfirmed.user",
      email: "unconfirmed.user@example.com",
      firstName: "Unconfirmed",
      lastName: "User",
      userRole: ["Worker"],
      dateCreated: "2023-03-10T00:00:00.000Z",
      lastLoggedIn: "2023-06-10T00:00:00.000Z",
      blocked: false,
      confirmed: false,
    },
  },
};

export const AllVariants: Story = {
  args: {
    user: {
      username: "all.variants",
      email: "all.variants@example.com",
      firstName: "All",
      lastName: "Variants",
      userRole: ["Worker", "Admin"],
      dateCreated: "2022-11-01T00:00:00.000Z",
      lastLoggedIn: "2023-04-01T00:00:00.000Z",
      blocked: true,
      confirmed: false,
    },
  },
};
