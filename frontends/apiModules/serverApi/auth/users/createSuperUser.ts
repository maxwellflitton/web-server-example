/**
 * createSuperUser.ts
 *
 * # Overview
 * Provides functionality to interact with the super admin user creation API endpoint 
 * (`/api/auth/v1/users/create/superadmin`). A successful creation returns a `201 Created` status.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when creating a super admin user.
 *
 * Fields:
 * - `email`: Validates that the email is in a proper format using Zod's built-in email validator.
 * - `password`: A simple string.
 * - `username`: A simple string.
 * - `first_name`: A simple string.
 * - `last_name`: A simple string.
 * - `user_role`: A simple string.
 */
const createSuperUserInputSchema = z.object({
  email: z.string().email(),
  username: z.string(),
  password: z.string(),
  first_name: z.string(),
  last_name: z.string(),
  user_role: z.enum([
    "Super Admin",
    "Admin",
    "Worker"
  ]),
});

// Export the input type inferred from the Zod schema.
export type CreateSuperUserInputSchema = z.infer<typeof createSuperUserInputSchema>;

/**
 * Zod schema for the output data upon successful super admin creation.
 * Currently defined as an empty object; adjust this schema as needed.
 */
const createSuperUserOutputSchema = z.object({}).strict();

// Export the output type inferred from the Zod schema.
export type CreateSuperUserOutputSchema = z.infer<typeof createSuperUserOutputSchema>;

/**
 * Type definition for the function that creates a new super admin user.
 */
export type CreateSuperUserFunction = ApiFunction<
  CreateSuperUserInputSchema,
  CreateSuperUserOutputSchema
>;

/**
 * Creates a new super admin user by sending a POST request to the designated endpoint.
 *
 * # Arguments
 * - `host`: The base URL of the API server.
 * - `user`: An object conforming to `CreateSuperUserInputSchema` with user details.
 *
 * # Returns
 * - A `Promise` resolving to `ApiResponse<CreateSuperUserOutputSchema>` if the call 
 *   returns a `201 Created` status, or an `ErrorResponse` otherwise.
 *
 * # Errors
 * - Returns an error response if the API call does not return `201 Created` or if 
 *   an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { createSuperUser } from "./path-to-module";
 *
 * const host = "http://localhost:3000";
 * const newSuperUser = {
 *   email: "admin@example.com",
 *   password: "securepassword",
 *   first_name: "Admin",
 *   last_name: "User",
 *   user_role: "Super Admin"
 * };
 *
 * createSuperUser(host, newSuperUser)
 *   .then(response => console.log("Super Admin created:", response))
 *   .catch(error => console.error("Error:", error));
 * ```
 */
export async function createSuperUser(user: CreateSuperUserInputSchema): Promise<ApiResponse<CreateSuperUserOutputSchema>> {
  const url = new UserUrl().createSuperUser;
  const params: HttpRequestParams = {
    url,
    httpMethod: "post",
    args: user,
  };

  const response = await httpRequest(params);

  if (response.status === 201) {
    const parsedOutput = createSuperUserOutputSchema.safeParse(response.body);
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
