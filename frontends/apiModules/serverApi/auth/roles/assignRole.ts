/**
 * assignRole.ts
 *
 * # Overview
 * This module provides functionality to interact with the user creation API endpoint using Axios.
 * It targets the `/api/auth/v1/users/create` endpoint and ensures that a successful user creation
 * returns a `201 Created` status. Robust error handling is included to manage unexpected responses.
 */

import { RoleUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when assigning a role.
 *
 * - `email`: Uses Zod’s built-in email validation.
 * - `role`: User's role.
 * - `jwt`: User's jwt.
 */
const assignRoleInputSchema = z.object({
  user_id: z.number(), // basic email validation
  role: z.enum([
    "Super Admin",
    "Admin",
    "Worker"
  ]),
  jwt: z.string()
});

/**
 * Zod schema for the output data after a user is created.
 * (Currently defined as an empty object; adjust as needed.)
 */
const assignRoleOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type AssignRoleInputSchema = z.infer<typeof assignRoleInputSchema>;
export type AssignRoleOutputSchema = z.infer<typeof assignRoleOutputSchema>;

/**
 * Type definition for the function that creates a new user.
 */
export type AssignRoleFunction = ApiFunction<AssignRoleInputSchema, AssignRoleOutputSchema>;

/**
 * Creates a new user by sending a POST request to the user creation endpoint.
 *
 * # Arguments
 * - `host`: The base URL of the API server.
 * - `user`: An object conforming to the `CreateUserInputSchema` interface containing the user's details.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<CreateUserOutputSchema>` if the user is created successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `201 Created` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { createUser, CreateUserInputSchema } from "./path-to-module";
 *
 * const host = "http://localhost:3000";
 * const user: CreateUserInputSchema = { email: "test@example.com", password: "securepassword" };
 *
 * createUser(host, user)
 *   .then(response => console.log("User created successfully:", response))
 *   .catch(error => console.error("Error creating user:", error));
 * ```
 */
export async function assignRole(input_data: AssignRoleInputSchema): Promise<ApiResponse<AssignRoleOutputSchema>> {
  const body = { user_id: input_data.user_id, role: input_data.role };

  const url = new RoleUrl().assignRole;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: body,
    jwt: input_data.jwt
  };

  const response = await httpRequest(params);

  // Validate the output schema with Zod
  if (response.status === 201) {
    const parsedOutput = assignRoleOutputSchema.safeParse(response.body);
    if (!parsedOutput.success) {
      return {
        status: 500,
        body: `Return body validation error - ${response.body} - ${parsedOutput.error.message}`,
      } as ErrorResponse;
    }
    return { status: response.status, body: parsedOutput.data };
  }

  return response as ErrorResponse;
}