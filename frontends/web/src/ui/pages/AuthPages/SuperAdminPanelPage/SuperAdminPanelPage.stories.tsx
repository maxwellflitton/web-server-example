/** @jsxImportSource @emotion/react */
import React from "react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter, Routes, Route } from "react-router-dom";

// Page under test
import { SuperAdminPanelPage } from "./SuperAdminPanelPage";

// Mocks
import { getAllUsers } from "#serverApi/auth/users/getAllUsers";
import { createUser } from "#serverApi/auth/users/createUser";
import { getJwt } from "#auth/localStorage";

const meta: Meta<typeof SuperAdminPanelPage> = {
  title: "Pages/SuperAdminPanelPage",
  component: SuperAdminPanelPage,
  parameters: {
    layout: "fullscreen",
    useCustomRouter: true,
  },
};

export default meta;
type Story = StoryObj<typeof SuperAdminPanelPage>;

/**
 * A helper that generates the new getAllUsers payload shape.
 * For each user, we'll produce both the `user` object
 * and a `role_permissions` array with their roles.
 */
function generateUserProfilesArray(length: number) {
  return Array.from({ length }, (_, index) => {
    // For every 3rd user, roles = ["Admin","Worker"], otherwise either ["Admin"] or ["Worker"]
    const roles =
      index % 3 === 0
        ? ["Admin"]
        : index % 3 === 1
        ? ["Worker"]
        : ["Super Admin"];

    return {
      user: {
        id: index + 1,
        confirmed: index % 4 === 0,
        username: `user_${index + 1}`,
        email: `user${index + 1}@example.com`,
        first_name: `First${index + 1}`,
        last_name: `Last${index + 1}`,
        user_role: roles[0], // The first role becomes the "user_role"
        date_created: `0${(index % 9 || 9)}/01/2022`, // e.g. 09/01/2022
        last_logged_in: `0${((index + 1) % 9 || 9)}/01/2022`,
        blocked: index % 3 === 0,
        uuid: `uuid-${index + 1}`,
      },
      role_permissions: roles.map((role, roleIndex) => ({
        id: index * 10 + roleIndex, // just a unique ID for story data
        user_id: index + 1,
        role,
      })),
    };
  });
}

/**
 * Wraps the story in a MemoryRouter with custom routes.
 */
const withCustomRouter = (
  initialEntries: Array<string | { pathname: string; state?: any }>
) => (StoryComponent: React.FC) =>
  (
    <MemoryRouter initialEntries={initialEntries}>
      <Routes>
        <Route path="/superadmin-panel" element={<StoryComponent />} />
        <Route
          path="/user-actions/:id"
          element={<div>Redirected to User Actions</div>}
        />
        <Route path="/create-user" element={<div>Redirected to Create User</div>} />
      </Routes>
    </MemoryRouter>
  );

/* ------------------------------------------------------------------
   Story: Default (Success) => 11 users, then after createUser => 12
-------------------------------------------------------------------- */
export const Default: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    // Simulate valid JWT
    getJwt.mockReturnValue("fake_jwt_token");

    // On initial mount, return 11 users
    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(11),
    });

    // When createUser is called, simulate success
    createUser.mockResolvedValue({
      status: 201,
      body: {},
    });

    // After successful createUser, fetchUsers is called again,
    // now returning 12 total (the new user is appended).
    const elevenUsersPlusNew = generateUserProfilesArray(11);
    elevenUsersPlusNew.push({
      user: {
        id: 12,
        confirmed: false,
        username: "newuser",
        email: "newuser@example.com",
        first_name: "New",
        last_name: "User",
        user_role: "Worker",
        date_created: "09/01/2022",
        last_logged_in: "09/01/2022",
        blocked: false,
        uuid: "uuid-12",
      },
      role_permissions: [
        {
          id: 999, // arbitrary
          user_id: 12,
          role: "Worker",
        },
      ],
    });

    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: elevenUsersPlusNew,
    });
  },
};

/* ----------------------------------------------------------
   Story: Loading (Delay the getAllUsers response for 3s)
------------------------------------------------------------ */
export const Loading: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    // Delay the response by 3s to show the loading spinner
    getAllUsers.mockImplementation(
      () =>
        new Promise((resolve) =>
          setTimeout(() => {
            resolve({
              status: 200,
              body: generateUserProfilesArray(11),
            });
          }, 3000)
        )
    );
  },
};

/* ----------------------------------------------------------
   Story: NetworkError (status = 0)
------------------------------------------------------------ */
export const NetworkError: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getAllUsers.mockResolvedValueOnce({
      status: 0,
      body: { Error: "Network error or server unreachable" },
    });
  },
};

/* ----------------------------------------------------------
   Story: ServerError (status = 500)
------------------------------------------------------------ */
export const ServerError: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getAllUsers.mockResolvedValueOnce({
      status: 500,
      body: "Internal server error",
    });
  },
};

/* ----------------------------------------------------------
   Story: EmptyState (no users returned)
------------------------------------------------------------ */
export const EmptyState: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: [], // no users
    });
  },
};

/* ------------------------------------------------------------------
   Story: CreateUserFailure => initial load is good, but create fails
-------------------------------------------------------------------- */
export const CreateUserFailure: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    // First call: 11 users
    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(11),
    });

    // createUser fails (e.g., 400 or 409, etc.)
    createUser.mockResolvedValue({
      status: 400,
      body: "Account creation failed due to validation error",
    });
  },
};

/* ------------------------------------------------------------------
   Story: MixedFailure => getAllUsers fails, createUser is set to succeed
-------------------------------------------------------------------- */
export const MixedFailure: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    // On first attempt to fetch users => server error
    getAllUsers.mockResolvedValueOnce({
      status: 500,
      body: "Internal server error",
    });

    // If the user tries to createUser, it 'succeeds'
    createUser.mockResolvedValue({
      status: 201,
      body: {},
    });

    // Then, if fetchUsers is called again => success with 11
    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(11),
    });
  },
};

/* ------------------------------------------------------------------
   Story: LessThanTen => returns 5 users
-------------------------------------------------------------------- */
export const LessThanTen: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(5),
    });
  },
};

/* ------------------------------------------------------------------
   Story: NoJwtFound => No JWT from the beginning.
   This tests if the component displays an error right away 
   (because it won't call getAllUsers without a valid JWT).
-------------------------------------------------------------------- */
export const NoJwtFound: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    // Simulate no JWT found at all
    getJwt.mockReturnValue(null);
  },
};

/* ------------------------------------------------------------------
   Story: NoJwtFoundDuringCreate => we DO have JWT for fetching users, 
   but not for createUser call (i.e., the second time getJwt is called).
-------------------------------------------------------------------- */
export const NoJwtFoundDuringCreate: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    // 1) For the initial fetch, we have a JWT
    getJwt
      .mockReturnValueOnce("fake_jwt_token") // for getAllUsers
      // 2) Then for the createUser call, we have no JWT
      .mockReturnValueOnce(null);

    // The initial fetch returns 11 users
    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(11),
    });

    // If createUser were to be called, it won't proceed
    // because the code checks getJwt first and sees null.
    // So the createUser.mockResolvedValue won't matter much,
    // but we'll still mock it to avoid any side effects.
    createUser.mockResolvedValue({
      status: 201,
      body: {},
    });
  },
};

/* ------------------------------------------------------------------
   Story: CreateUserFailureWithDelay => initial load is fine, 
   but the createUser call takes a few seconds and ultimately fails
-------------------------------------------------------------------- */
export const CreateUserFailureWithDelay: Story = {
  decorators: [withCustomRouter(["/superadmin-panel"])],
  async beforeEach() {
    getJwt.mockReturnValue("fake_jwt_token");

    // First call: 11 users
    getAllUsers.mockResolvedValueOnce({
      status: 200,
      body: generateUserProfilesArray(11),
    });

    // createUser is delayed by 3s and then returns a failure (e.g. 400)
    createUser.mockImplementation(
      () =>
        new Promise((resolve) => {
          setTimeout(() => {
            resolve({
              status: 400,
              body: "Account creation failed due to validation error",
            });
          }, 3000);
        })
    );
  },
};

