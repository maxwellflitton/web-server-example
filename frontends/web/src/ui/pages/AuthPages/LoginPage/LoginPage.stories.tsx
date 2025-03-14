/** @jsxImportSource @emotion/react */
import React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter, Routes, Route } from "react-router-dom";
import { LoginPage } from "./LoginPage";
import { loginUser } from "#serverApi/auth/auth/login";

const meta: Meta<typeof LoginPage> = {
  title: "Pages/LoginPage",
  component: LoginPage,
  parameters: {
    layout: "fullscreen",
    useCustomRouter: true,
  },
};

export default meta;
type Story = StoryObj<typeof LoginPage>;

/**
 * Wraps the story in a MemoryRouter and defines routes for successful navigation.
 */
const withCustomRouter = (
  initialEntries: Array<string | { pathname: string; state?: any }>
) => (StoryComponent: React.FC) => (
  <MemoryRouter initialEntries={initialEntries}>
    <Routes>
      <Route path="/login" element={<StoryComponent />} />
      <Route
        path="/superadmin-panel"
        element={<div>Redirected to Superadmin Panel</div>}
      />
      <Route
        path="/admin-panel"
        element={<div>Redirected to Admin Panel</div>}
      />
      <Route
        path="/worker-panel"
        element={<div>Redirected to Worker Panel</div>}
      />
    </Routes>
  </MemoryRouter>
);

// Default (Successful Login Response)
// (By default, the LoginForm sets role to "Worker". Change the role manually in the form for testing other routes.)
export const Default: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 200,
      body: {},
    });
  },
};

// Network Error
export const NetworkError: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 0,
      body: { Error: "Network error or server unreachable" },
    });
  },
};

// Rate Limit Error
export const RateLimitError: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 500,
      body: "Rate limit exceeded for this email. Please try again later.",
    });
  },
};

// Server Error
export const ServerError: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 500,
      body: "Internal server error",
    });
  },
};

// User lacks the required role
export const UserDoesNotHaveRequiredRole: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 403,
      body: "User does not have the required role",
    });
  },
};

// Invalid password
export const InvalidPassword: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 401,
      body: "Invalid password",
    });
  },
};

// User is not confirmed
export const UserNotConfirmed: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 403,
      body: "User is not confirmed",
    });
  },
};

// User is blocked
export const UserBlocked: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockResolvedValue({
      status: 403,
      body: "User is blocked",
    });
  },
};


// Loading State
export const LoadingState: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    loginUser.mockImplementation(
      () =>
        new Promise((resolve) =>
          setTimeout(() => resolve({ status: 200, body: {} }), 3000)
        )
    );
  },
};

// Multiple Submission After Error
export const MultipleSubmissionAfterError: Story = {
  decorators: [withCustomRouter(["/login"])],
  async beforeEach() {
    let callCount = 0;
    loginUser.mockImplementation(() => {
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
        status: 200,
        body: {},
      });
    });
  },
};
