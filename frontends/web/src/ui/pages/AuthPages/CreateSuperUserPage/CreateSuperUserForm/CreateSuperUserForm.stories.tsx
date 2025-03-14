import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { CreateSuperUserForm } from "./CreateSuperUserForm";


const meta: Meta<typeof CreateSuperUserForm> = {
  title: "PageComponents/CreateSuperUser/CreateSuperUserForm",
  component: CreateSuperUserForm,
  parameters: {
    layout: "centered", 
  },
  args: {
    apiError: "",
    apiFailure: false,
    submitCreateSuperUserForm: action("submitCreateSuperUserForm"),
  },
  decorators: [
    (Story) => (
      <div style={{ width: "80%", margin: "20% auto"}}>
        <Story />
      </div>
    ),
  ],
};

export default meta;

type Story = StoryObj<typeof CreateSuperUserForm>;


export const Default: Story = {
  args: {
  },
};


export const WithApiFailure: Story = {
  args: {
    apiFailure: true,
    apiError: "Unable to create account. Please try again later.",
  },
};
