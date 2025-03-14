import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when retrieving a user.
 *
 * - `jwt`: A string containing the JSON Web Token for authentication.
 * - `url`: A string representing the endpoint URL to fetch the user data.
 */
const getUserInputSchema = z.object({
  jwt: z.string().optional(),
  url: z.string(),
});

/**
 * Zod enum schema for the user role.
 *
 * Valid values are:
 * - `Admin`
 * - `Worker`
 * - `Super Admin`
 * - `Unreachable`
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
 * - `id`: The user's numerical ID.
 * - `confirmed`: Boolean indicating whether the user's email is confirmed.
 * - `username`: The user's username.
 * - `email`: The user's email address.
 * - `first_name`: The user's first name.
 * - `last_name`: The user's last name.
 * - `user_role`: The user's role (as a `UserRoleSchema` enum).
 * - `date_created`: The timestamp when the user was created (as a string).
 * - `last_logged_in`: The timestamp when the user last logged in (as a string).
 * - `blocked`: Boolean indicating if the user is blocked.
 * - `uuid`: The user's UUID.
 */
const TrimmedUserSchema = z.object({
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
}).strict();

/**
 * Zod schema for a user profile object, which includes a trimmed user and
 * an array of role permissions.
 *
 * - `user`: A `TrimmedUserSchema` representing user details.
 * - `role_permissions`: An array of `RolePermissionSchema` objects,
 *    describing the permissions assigned to the user's role.
 */
const getUserOutputSchema = z.object({
  user: TrimmedUserSchema,
  roles: z.array(UserRoleSchema),
}).strict();

/**
 * Exported TypeScript types inferred from the Zod schemas.
 */
export type UserRole = z.infer<typeof UserRoleSchema>;
export type TrimmedUser = z.infer<typeof TrimmedUserSchema>;

export type GetUserInputSchema = z.infer<typeof getUserInputSchema>;
export type GetUserOutputSchema = z.infer<typeof getUserOutputSchema>;

/**
 * Type definition for the function that retrieves a user by UUID.
 */
export type GetUserFunction = ApiFunction<GetUserInputSchema, GetUserOutputSchema>;

/**
 * Retrieves a user by sending a GET request to the user retrieval endpoint.
 *
 * # Arguments
 * - `data`: An object conforming to the `GetUserInputSchema` interface containing the
 *   JWT token and the URL to fetch the user data.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<GetUserOutputSchema>` if the user is retrieved successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `200 OK` status or if
 *   an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { getUser, GetUserInputSchema } from "./path-to-module";
 * import { UserUrl } from "./path-to-module";
 *
 * let url = new UserUrl().constructGetUserById(2);
 *
 * const data: GetUserInputSchema = { url, jwt: "some-token"};
 *
 * getUser(data)
 *   .then(response => console.log("User retrieved successfully:", response))
 *   .catch(error => console.error("Error retrieving user:", error));
 * ```
 *
 * We can construct different URLs with the following code:
 * ```typescript
 * import { UserUrl } from "./path-to-module";
 *
 * let url = new UserUrl().constructGetUserById(2);
 * let url = new UserUrl().constructGetUserByEmail("test@gmail.com");
 * let url = new UserUrl().constructGetUserByUuid("some-uuid");
 * ```
 */
export async function getUser(
  data: GetUserInputSchema
): Promise<ApiResponse<GetUserOutputSchema>> {
  const params: HttpRequestParams = {
    url: data.url,
    httpMethod: "get",
    ...(data.jwt ? { jwt: data.jwt } : {}),
  };

  const response = await httpRequest(params);

  // Validate the output schema with Zod
  if (response.status === 200) {
    const parsedOutput = getUserOutputSchema.safeParse(response.body);
    if (!parsedOutput.success) {
      return {
        status: 500,
        body: `Return body validation error - ${parsedOutput.error.message}`,
      } as ErrorResponse;
    }
    return { status: response.status, body: response.body };
  }
  return response as ErrorResponse;
}
