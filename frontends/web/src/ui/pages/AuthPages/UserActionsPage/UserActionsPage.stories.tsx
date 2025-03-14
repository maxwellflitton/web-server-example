/** @jsxImportSource @emotion/react */
import React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter, Routes, Route } from "react-router-dom";

// Component under test
import { UserActionsPage } from "./UserActionsPage";

// Mocks
import { getUser } from "#serverApi/auth/users/getUser";
import { blockUser } from "#serverApi/auth/users/blockUser";
import { unblockUser } from "#serverApi/auth/users/unblockUser";
import { resendConfirmationEmail } from "#serverApi/auth/auth/resendConfirmationEmail";
import { deleteUser } from "#serverApi/auth/users/deleteUser";
import { updateRoles } from "#serverApi/auth/roles/updateRoles";
import { getJwt } from "#auth/localStorage";

// Simple sleep helper for artificial delays
function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const meta: Meta<typeof UserActionsPage> = {
  title: "Pages/UserActionsPage",
  component: UserActionsPage,
  parameters: {
    layout: "fullscreen",
    useCustomRouter: true,
  },
};
export default meta;

type Story = StoryObj<typeof UserActionsPage>;

// -----------------------
// Common mock data
// -----------------------
const mockUser = {
  id: 123,
  confirmed: false,
  username: "mockUser",
  email: "mock@example.com",
  first_name: "Mock",
  last_name: "User",
  date_created: "2025-01-01T12:00:00",
  last_logged_in: "2025-02-01T18:00:00",
  blocked: false,
  uuid: "some-uuid-here",
};
const mockRoles = ["Worker"];

// Helper to wrap stories in a MemoryRouter
const withCustomRouter = (initialEntries: Array<string>) => (StoryComponent: React.FC) => (
  <MemoryRouter initialEntries={initialEntries}>
    <Routes>
      <Route path="/user-actions/:userId?" element={<StoryComponent />} />
      <Route path="/superadmin-panel" element={<div>Redirected to Super Admin Panel</div>} />
    </Routes>
  </MemoryRouter>
);

// -----------------------
// 1) Basic GET / fetchUser Scenarios
// -----------------------
export const Default: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // Simulate a valid JWT
    getJwt.mockReturnValue("fake-jwt-token");
    // 1) getUser => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });

    // Provide default success responses for all "edit" calls
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

export const NetworkErrorGetUser: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // 1) getUser => network error
    getUser.mockResolvedValueOnce({
      status: 0,
      body: "Network error or server unreachable",
    });
  },
};

export const ServerErrorGetUser: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // 1) getUser => server error
    getUser.mockResolvedValueOnce({
      status: 500,
      body: "Internal server error",
    });
  },
};

export const MissingJwt: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue(null);
    // If code tries calling getUser
    getUser.mockResolvedValueOnce({
      status: 500,
      body: "No JWT found",
    });
  },
};

export const NoUserId: Story = {
  decorators: [withCustomRouter(["/user-actions"])],
  async beforeEach() {
    // No userId => "User ID not provided..."
    getJwt.mockReturnValue("fake-jwt-token");
  },
};

export const UserNotFound: Story = {
  decorators: [withCustomRouter(["/user-actions/999"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // Suppose we get user=null from server
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: null, roles: [] },
    });
  },
};

/** 
 * We can delay the getUser call by 3s to show the loading spinner
 */
export const LoadingGetUser: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockImplementation(() => {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            status: 200,
            body: { user: mockUser, roles: mockRoles },
          });
        }, 3000);
      });
    });
  },
};

// -----------------------
// 2) Block / Unblock Scenarios
//    (Second getUser call if block/unblock => success)
// -----------------------

/**
 * blockUser => success => second getUser => success
 */
export const BlockUserRefetchSuccess: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // 1) initial getUser => success, user not blocked
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: {
        user: { ...mockUser, blocked: false },
        roles: mockRoles,
      },
    });
    // 2) blockUser => success
    blockUser.mockResolvedValue({ status: 200, body: {} });
    // 3) second getUser => also success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: {
        user: { ...mockUser, blocked: true },
        roles: mockRoles,
      },
    });
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * blockUser => success => second getUser => network error
 */
export const BlockUserRefetchNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block => success
    blockUser.mockResolvedValue({ status: 200, body: {} });
    // second getUser => network error
    getUser.mockResolvedValueOnce({
      status: 0,
      body: "Network error or server unreachable",
    });
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * blockUser => success => second getUser => server error
 */
export const BlockUserRefetchServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // initial => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block => success
    blockUser.mockResolvedValue({ status: 200, body: {} });
    // second => server error
    getUser.mockResolvedValueOnce({
      status: 500,
      body: "Server error",
    });
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * blockUser => success => second getUser => delayed => success 
 * => overlay spinner for ~2s
 */
export const BlockUserRefetchDelay: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block => success
    blockUser.mockResolvedValue({ status: 200, body: {} });
    // second => delayed success
    getUser.mockImplementationOnce(
      () =>
        new Promise((resolve) =>
          setTimeout(() => {
            resolve({
              status: 200,
              body: { user: { ...mockUser, blocked: true }, roles: mockRoles },
            });
          }, 2000)
        )
    );
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * blockUser => network error => skip refetch
 */
export const BlockUserNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block => network error => no second getUser
    blockUser.mockResolvedValue({ status: 0, body: "Network error" });
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * blockUser => server error => skip refetch
 */
export const BlockUserServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block => server error => skip second getUser
    blockUser.mockResolvedValue({ status: 500, body: "Server error" });
    // Defaults
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * Similarly we can have "UnblockUser..." scenarios if you want 
 * the user to start out blocked, then do success/fail/delay on refetch
 * (see below for an example)
 */
export const UnblockUserRefetchSuccess: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // user starts blocked
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: true }, roles: mockRoles },
    });
    // unblock => success
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    // second => success => user is now unblocked
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, blocked: false }, roles: mockRoles },
    });
    // block safe defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

// -----------------------
// 3) Update Roles Scenarios
// -----------------------

/**
 * updateRoles => success => second getUser => success
 */
export const UpdateRolesRefetchSuccess: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => success
    updateRoles.mockResolvedValue({ status: 201, body: {} });
    // second => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: {
        user: { ...mockUser, roles: [...mockRoles, "Admin"] },
        roles: [...mockRoles, "Admin"],
      },
    });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * updateRoles => success => second getUser => network error
 */
export const UpdateRolesRefetchNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => success
    updateRoles.mockResolvedValue({ status: 201, body: {} });
    // second => network error
    getUser.mockResolvedValueOnce({
      status: 0,
      body: "Network error on refetch",
    });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * updateRoles => success => second getUser => server error
 */
export const UpdateRolesRefetchServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => success
    updateRoles.mockResolvedValue({ status: 201, body: {} });
    // second => server error
    getUser.mockResolvedValueOnce({
      status: 500,
      body: "Internal server error on refetch",
    });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * updateRoles => success => second getUser => delayed
 */
export const UpdateRolesRefetchDelay: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => success
    updateRoles.mockResolvedValue({ status: 201, body: {} });
    // second => delayed success
    getUser.mockImplementationOnce(
      () =>
        new Promise((resolve) =>
          setTimeout(() => {
            resolve({
              status: 200,
              body: {
                user: { ...mockUser, roles: ["Worker", "Admin"] },
                roles: ["Worker", "Admin"],
              },
            });
          }, 2000)
        )
    );
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * updateRoles => network error => no second getUser
 */
export const UpdateRolesNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => network error
    updateRoles.mockResolvedValue({ status: 0, body: "Network error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

/**
 * updateRoles => server error => no second getUser
 */
export const UpdateRolesServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    // initial => success
    getJwt.mockReturnValue("fake-jwt-token");
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // update => server error
    updateRoles.mockResolvedValue({ status: 500, body: "Server error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
  },
};

// -----------------------
// 4) Resend Confirmation
// (no second getUser call needed)
// -----------------------
export const ResendConfirmationSuccess: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success, not confirmed
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, confirmed: false }, roles: mockRoles },
    });
    // resend => success
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

export const ResendConfirmationNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success, not confirmed
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, confirmed: false }, roles: mockRoles },
    });
    // resend => network error
    resendConfirmationEmail.mockResolvedValue({ status: 0, body: "Network error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

export const ResendConfirmationServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success, not confirmed
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: { ...mockUser, confirmed: false }, roles: mockRoles },
    });
    // resend => server error
    resendConfirmationEmail.mockResolvedValue({ status: 500, body: "Server error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

// -----------------------
// 5) Delete User Scenarios
// (no second getUser call needed, but navigates on success)
// -----------------------
export const DeleteUserSuccess: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // delete => success => triggers navigate
    deleteUser.mockResolvedValue({ status: 201, body: {} });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

export const DeleteUserNetworkError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // delete => network error => skip navigate
    deleteUser.mockResolvedValue({ status: 0, body: "Network error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};

export const DeleteUserServerError: Story = {
  decorators: [withCustomRouter(["/user-actions/123"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake-jwt-token");
    // user => success
    getUser.mockResolvedValueOnce({
      status: 200,
      body: { user: mockUser, roles: mockRoles },
    });
    // delete => server error => skip navigate
    deleteUser.mockResolvedValue({ status: 500, body: "Server error" });
    // Defaults
    blockUser.mockResolvedValue({ status: 200, body: {} });
    unblockUser.mockResolvedValue({ status: 200, body: {} });
    resendConfirmationEmail.mockResolvedValue({ status: 200, body: {} });
    updateRoles.mockResolvedValue({ status: 201, body: {} });
  },
};
