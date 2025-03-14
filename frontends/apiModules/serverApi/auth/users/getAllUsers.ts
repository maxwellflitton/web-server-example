/**
 * getAllUsers.ts
 *
 * # Overview
 * This module provides functionality to interact with the user retrieval API endpoint using Axios.
 * It targets the /api/auth/v1/users/getAllUsers endpoint and ensures that a successful retrieval
 * returns a 200 OK status. Robust error handling is included to manage unexpected responses.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunctionNoInput, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod enum schema for the user role.
 *
 * Valid values are:
 * - Admin
 * - Worker
 * - Super Admin
 * - Unreachable
 */
const UserRoleSchema = z.enum([
  "Admin",
  "Worker",
  "Super Admin",
  "Unreachable",
]);

/**
 * Zod schema for a trimmed user object, representing minimal user details.
 *
 * - id: The user's numerical ID.
 * - confirmed: Boolean indicating whether the user's email is confirmed.
 * - username: The user's username.
 * - email: The user's email address.
 * - first_name: The user's first name.
 * - last_name: The user's last name.
 * - user_role: The user's role (as a UserRoleSchema enum).
 * - date_created: The timestamp when the user was created (as a string).
 * - last_logged_in: The timestamp when the user last logged in (as a string).
 * - blocked: Boolean indicating if the user is blocked.
 * - uuid: The user's UUID.
 */
const TrimmedUserSchema = z
  .object({
    id: z.number(),
    confirmed: z.boolean(),
    username: z.string(),
    email: z.string().email(),
    first_name: z.string(),
    last_name: z.string(),
    user_role: UserRoleSchema,
    date_created: z.string(),
    last_logged_in: z.string(),
    blocked: z.boolean(),
    uuid: z.string().uuid(),
  })
  .strict();

/**
 * Zod schema for a role permission object.
 *
 * - id: The unique identifier for the role permission entry.
 * - user_id: The ID of the user.
 * - role: The role assigned to the user (as a UserRoleSchema enum).
 */
const RolePermissionSchema = z
  .object({
    id: z.number(),
    user_id: z.number(),
    role: UserRoleSchema,
  })
  .strict();

/**
 * Zod schema for a user profile object, which includes a trimmed user and
 * an array of role permission objects.
 *
 * - user: A TrimmedUserSchema representing user details.
 * - role_permissions: An array of RolePermissionSchema objects describing the user's permissions.
 */
const UserProfileSchema = z
  .object({
    user: TrimmedUserSchema,
    role_permissions: z.array(RolePermissionSchema),
  })
  .strict();

/**
 * Zod schema representing the output of the "get all users" endpoint:
 * An array of user profile objects.
 */
const getAllUsersOutputSchema = z.array(UserProfileSchema);

// Types inferred from zod schemas
export type RolePermission = z.infer<typeof RolePermissionSchema>;
export type UserProfileObject = z.infer<typeof UserProfileSchema>;
export type GetAllUsersOutputSchema = z.infer<typeof getAllUsersOutputSchema>;
export type GetAllUsersFunction = ApiFunctionNoInput<GetAllUsersOutputSchema>;

/**
 * Retrieves all users from the system. Requires super admin authorization.
 *
 * # Returns
 * - A Promise resolving to an ApiResponse<GetAllUsersOutputSchema> containing an array of user
 *   profile objects if successful. Otherwise, resolves to an ErrorResponse.
 *
 * # Errors
 * - Returns an error response if the API call doesn't return a 200 OK status.
 * - Returns a 500 error if the response doesn't match the expected schema.
 * - Returns a 401 error if the user isn't authorized as a super admin.
 */
export async function getAllUsers(jwt: string): Promise<ApiResponse<GetAllUsersOutputSchema>> {
  const url = new UserUrl().getAllUsers;
  const params: HttpRequestParams = {
    url,
    httpMethod: "get",
    jwt,
  };

  const response = await httpRequest(params);

  if (response.status === 200) {
    const parsedOutput = getAllUsersOutputSchema.safeParse(response.body);
    if (!parsedOutput.success) {
      return {
        status: 500,
        body: `Return body validation error - ${parsedOutput.error.message}`,
      } as ErrorResponse;
    }
    return { status: response.status, body: parsedOutput.data };
  }

  return response as ErrorResponse;
}
