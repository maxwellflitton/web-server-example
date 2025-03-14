/** @jsxImportSource @emotion/react */
import React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter, Routes, Route } from "react-router-dom";
import { CreateSuperUserPage } from "./CreateSuperUserPage";
import { createSuperUser } from "#serverApi/auth/users/createSuperUser";

const meta: Meta<typeof CreateSuperUserPage> = {
  title: "Pages/CreateSuperUserPage",
  component: CreateSuperUserPage,
  parameters: {
    layout: "fullscreen",
    useCustomRouter: true,
  },
};

export default meta;
type Story = StoryObj<typeof CreateSuperUserPage>;

/**
 * Wraps the story in a MemoryRouter and defines a route for success navigation.
 */
const withCustomRouter = (
  initialEntries: Array<string | { pathname: string; state?: any }>
) => (Story: React.FC) => (
  <MemoryRouter initialEntries={initialEntries}>
    <Routes>
      <Route path="/create-super-user" element={<Story />} />
      <Route
        path="/login"
        element={<div>Redirected to Login</div>}
      />
    </Routes>
  </MemoryRouter>
);

// Default (Success Response)
export const Default: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    createSuperUser.mockResolvedValue({
      status: 201,
      body: {},
    });
  },
};

// Missing User State
export const MissingUserState: Story = {
  decorators: [withCustomRouter(["/create-super-user"])],
};

// Network Error
export const NetworkError: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    createSuperUser.mockResolvedValue({
      status: 0,
      body: { Error: "Network error or server unreachable" },
    });
  },
};

// Rate Limit Error
export const RateLimitError: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    createSuperUser.mockResolvedValue({
      status: 500,
      body: "Rate limit exceeded for this email. Please try again later.",
    });
  },
};

// Server Error
export const ServerError: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    createSuperUser.mockResolvedValue({
      status: 500,
      body: "Internal server error",
    });
  },
};

// Loading State
export const LoadingState: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    createSuperUser.mockImplementation(() =>
      new Promise((resolve) =>
        setTimeout(() => resolve({ status: 201, body: {} }), 3000)
      )
    );
  },
};

// Multiple Submission After Error
export const MultipleSubmissionAfterError: Story = {
  decorators: [
    withCustomRouter([
      {
        pathname: "/create-super-user",
        state: { user: { email: "test@example.com", password: "password123" } },
      },
    ]),
  ],
  async beforeEach() {
    let callCount = 0;
    createSuperUser.mockImplementation(() => {
      callCount++;
      if (callCount === 1) {
        return Promise.resolve({
          status: 500,
          body: "First attempt fails",
        });
      }
      if (callCount === 2) {
        return Promise.resolve({
          status: 0,
          body: { Error: "Second attempt fails" },
        });
      }
      return Promise.resolve({
        status: 201,
        body: {},
      });
    });
  },
};
