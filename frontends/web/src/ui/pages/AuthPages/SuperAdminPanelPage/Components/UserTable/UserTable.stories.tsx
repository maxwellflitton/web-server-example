import * as React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { UserTable } from "./UserTable";
import { columns, User } from "./Columns";


// Create sample data with more than 10 rows to test pagination,
// and assign multiple roles to some users.
const data: User[] = Array.from({ length: 11 }, (_, index) => ({
  id: (index + 1).toString(),
  username: `user_${index + 1}`,
  email: `user${index + 1}@example.com`,
  firstName: `First${index + 1}`,
  lastName: `Last${index + 1}`,
  roles:
  index % 3 === 0
    ? ["Admin"]
    : index % 3 === 1
    ? ["Worker"]
    : ["Super Admin"],
  dateCreated: `0${((index % 9) || 9)}/01/2022`,
  lastLoggedIn: `0${(((index + 1) % 9) || 9)}/01/2022`,
  blocked: index % 3 === 0,
  confirmed: index % 4 === 0,
}));

const meta: Meta<typeof UserTable> = {
  title: "PageComponents/User/UserTable",
  component: UserTable,
  parameters: {
    layout: "centered",
  },
  args: {
    columns,
    data,
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
type Story = StoryObj<typeof UserTable>;

export const Default: Story = {
  args: {},
};
