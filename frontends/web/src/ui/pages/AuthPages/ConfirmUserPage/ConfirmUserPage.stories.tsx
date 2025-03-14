/** @jsxImportSource @emotion/react */
import React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter, Routes, Route } from "react-router-dom";

import { ConfirmUserPage } from "./ConfirmUserPage";

// These come from your .mock.ts files or wherever your storybook mocks are set up
import { getUser } from "#serverApi/auth/users/getUser";
import { confirmUser } from "#serverApi/auth/users/confirmUser";
import { resetPassword } from "#serverApi/auth/users/resetPassword";
import { getJwt } from "#auth/localStorage";

const meta: Meta<typeof ConfirmUserPage> = {
  title: "Pages/ConfirmUserPage",
  component: ConfirmUserPage,
  parameters: {
    layout: "fullscreen",
    useCustomRouter: true,
  },
};

export default meta;
type Story = StoryObj<typeof ConfirmUserPage>;

/**
 * A custom router decorator that defines:
 * - a route with the uuid param (for normal cases)
 * - a route without the uuid (for MissingUuid)
 * - a route for the "/login" navigation target (when confirmation is successful)
 */
const withCustomRouter = (
  initialEntries: Array<string | { pathname: string; state?: any }>
) => (Story: React.FC) =>
  (
    <MemoryRouter initialEntries={initialEntries}>
      <Routes>
        <Route path="/confirm-user/:uuid" element={<Story />} />
        <Route path="/confirm-user" element={<Story />} />
        <Route path="/login" element={<div>Redirected to Login</div>} />
      </Routes>
    </MemoryRouter>
  );

export const Default: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    // Always return a mock JWT so the fetchUser logic proceeds
    getJwt.mockReturnValue("fake_jwt_token");

    getUser.mockResolvedValue({
      status: 200,
      body: {
        user: {
          id: 1,
          username: "johndoe",
          email: "john@example.com",
          first_name: "John",
          last_name: "Doe",
          user_role: "user",
          date_created: "2025-01-01T00:00:00Z",
          last_logged_in: "2025-01-02T00:00:00Z",
          blocked: false,
          uuid: "123e4567-e89b-12d3-a456-426614174000",
        },
      },
    });
    confirmUser.mockResolvedValue({
      status: 201,
      body: {},
    });
    resetPassword.mockResolvedValue({
      status: 201,
      body: {},
    });
  },
};

export const MissingUuid: Story = {
  decorators: [withCustomRouter(["/confirm-user"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");
    // No API calls are made because the component detects the missing uuid immediately.
  },
};

export const LoadingState: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    // Delay getUser response to show the loading spinner
    getUser.mockImplementation(
      () =>
        new Promise((resolve) =>
          setTimeout(
            () =>
              resolve({
                status: 200,
                body: {
                  user: {
                    id: 1,
                    username: "johndoe",
                    email: "john@example.com",
                    first_name: "John",
                    last_name: "Doe",
                    user_role: "user",
                    date_created: "2025-01-01T00:00:00Z",
                    last_logged_in: "2025-01-02T00:00:00Z",
                    blocked: false,
                    uuid: "123e4567-e89b-12d3-a456-426614174000",
                  },
                },
              }),
            5000
          )
        )
    );
    confirmUser.mockResolvedValue({
      status: 201,
      body: {},
    });
    resetPassword.mockResolvedValue({
      status: 201,
      body: {},
    });
  },
};

export const GetUserFailure: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getUser.mockResolvedValue({
      status: 404,
      body: "User not found",
    });
    // confirmUser and resetPassword calls won't happen because the form is disabled.
  },
};

export const ConfirmUserApiError: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getUser.mockResolvedValue({
      status: 200,
      body: {
        user: {
          id: 1,
          username: "johndoe",
          email: "john@example.com",
          first_name: "John",
          last_name: "Doe",
          user_role: "user",
          date_created: "2025-01-01T00:00:00Z",
          last_logged_in: "2025-01-02T00:00:00Z",
          blocked: false,
          uuid: "123e4567-e89b-12d3-a456-426614174000",
        },
      },
    });
    confirmUser.mockResolvedValue({
      status: 0,
      body: { Error: "Network error or server unreachable" },
    });
    // resetPassword will not be called if confirmUser fails first.
  },
};

export const ResetPasswordApiError: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getUser.mockResolvedValue({
      status: 200,
      body: {
        user: {
          id: 1,
          username: "johndoe",
          email: "john@example.com",
          first_name: "John",
          last_name: "Doe",
          user_role: "user",
          date_created: "2025-01-01T00:00:00Z",
          last_logged_in: "2025-01-02T00:00:00Z",
          blocked: false,
          uuid: "123e4567-e89b-12d3-a456-426614174000",
        },
      },
    });
    confirmUser.mockResolvedValue({
      status: 201,
      body: {},
    });
    resetPassword.mockResolvedValue({
      status: 0,
      body: { Error: "Network error or server unreachable" },
    });
  },
};

export const MultipleSubmissionAfterError: Story = {
  decorators: [
    withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
  ],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getUser.mockResolvedValue({
      status: 200,
      body: {
        user: {
          id: 1,
          username: "johndoe",
          email: "john@example.com",
          first_name: "John",
          last_name: "Doe",
          user_role: "user",
          date_created: "2025-01-01T00:00:00Z",
          last_logged_in: "2025-01-02T00:00:00Z",
          blocked: false,
          uuid: "123e4567-e89b-12d3-a456-426614174000",
        },
      },
    });

    let callCount = 0;
    confirmUser.mockImplementation(() => {
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
          body: { Error: "Second attempt fails (network)" },
        });
      }
      return Promise.resolve({
        status: 201,
        body: {},
      });
    });

    // We'll just make resetPassword succeed on its first call here.
    resetPassword.mockResolvedValue({
      status: 201,
      body: {},
    });
  },
};

export const JwtFailure: Story = {
    decorators: [
      withCustomRouter(["/confirm-user/123e4567-e89b-12d3-a456-426614174000"]),
    ],
    async beforeEach() {
      // Simulate JWT failure by returning null
      getJwt.mockReturnValue(null);
    },
  };