/**
 * deleteUser.ts
 *
 * # Overview
 * This module provides functionality to delete a user via the API endpoint.
 * It targets the `/api/auth/v1/users/delete` endpoint and requires SuperAdmin authorization.
 * A successful deletion returns a `200 OK` status.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when deleting a user.
 *
 * - `user_id`: The user id
 */
const deleteUserInputSchema = z.object({
  user_id: z.number()
});

/**
 * Zod schema for the output data after user deletion.
 * Returns an empty object on success.
 */
const deleteUserOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas
export type DeleteUserInputSchema = z.infer<typeof deleteUserInputSchema>;
export type DeleteUserOutputSchema = z.infer<typeof deleteUserOutputSchema>;

/**
 * Type definition for the function that deletes a user.
 */
export type DeleteUserFunction = ApiFunction<DeleteUserInputSchema, DeleteUserOutputSchema>;

/**
 * Deletes a user by sending a DELETE request to the user deletion endpoint.
 * Requires SuperAdmin authorization via JWT token.
 *
 * # Arguments
 * - `params`: An object conforming to `DeleteUserInputSchema` containing:
 *   - The UUID of the user to delete
 *   - The SuperAdmin's JWT token
 *
 * # Returns
 * - A `Promise` resolving to `ApiResponse<DeleteUserOutputSchema>` if successful,
 *   or an `ErrorResponse` if the deletion fails or authorization is invalid.
 *
 * # Errors
 * - Returns a 401 error if the token is missing or invalid
 * - Returns a 403 error if the token is not from a SuperAdmin
 * - Returns a 404 error if the user UUID is not found
 * - Returns a 500 error for other unexpected failures
 *
 * # Usage
 * ```typescript
 * import { deleteUser } from "./deleteUser";
 *
 * const params = {
 *   uuid: "123e4567-e89b-12d3-a456-426614174000",
 *   token: "jwt-token-here"
 * };
 *
 * deleteUser(params)
 *   .then(response => console.log("User deleted successfully"))
 *   .catch(error => console.error("Failed to delete user:", error));
 * ```
 */
export async function deleteUser(params: DeleteUserInputSchema, jwt: string): Promise<ApiResponse<DeleteUserOutputSchema>> {
  const url = new UserUrl().deleteUser;
  const httpParams: HttpRequestParams = {
    url,
    httpMethod: "post",
    args: params,
    jwt: jwt,
  };

  const response = await httpRequest(httpParams);

  if (response.status === 201) {
    const parsedOutput = deleteUserOutputSchema.safeParse(response.body);
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

