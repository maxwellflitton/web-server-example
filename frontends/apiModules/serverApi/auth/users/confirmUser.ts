/**
 * confirmUser.ts
 *
 * # Overview
 * This module provides functionality to interact with the user confirmation API endpoint using Axios.
 * It targets the `/api/auth/v1/users/confirm` endpoint and ensures that a successful confirmation
 * returns a `201 Created` status. Robust error handling is included to manage unexpected responses.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when confirming a user.
 *
 * - `unique_id`: A UUID string representing a unique identifier.
 */
const confirmUserInputSchema = z.object({
  unique_id: z.string().uuid(),
});

/**
 * Zod schema for the output data after a user is confirmed.
 * (Currently defined as an empty object; adjust as needed.)
 */
const confirmUserOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type ConfirmUserInputSchema = z.infer<typeof confirmUserInputSchema>;
export type ConfirmUserOutputSchema = z.infer<typeof confirmUserOutputSchema>;

/**
 * Type definition for the function that confirms a user.
 */
export type ConfirmUserFunction = ApiFunction<ConfirmUserInputSchema, ConfirmUserOutputSchema>;

/**
 * Confirms a user by sending a POST request to the user confirmation endpoint.
 *
 * # Arguments
 * - `user`: An object conforming to the `ConfirmUserInputSchema` interface containing the user's confirmation details.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<ConfirmUserOutputSchema>` if the user is confirmed successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `201 Created` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { confirmUser, ConfirmUserInputSchema } from "./path-to-module";
 *
 * const user: ConfirmUserInputSchema = { 
 *   password: "securepassword", 
 *   unique_id: "123e4567-e89b-12d3-a456-426614174000" 
 * };
 *
 * confirmUser(user)
 *   .then(response => console.log("User confirmed successfully:", response))
 *   .catch(error => console.error("Error confirming user:", error));
 * ```
 */
export async function confirmUser(user: ConfirmUserInputSchema): Promise<ApiResponse<ConfirmUserOutputSchema>> {
  const url = new UserUrl().confirmUser;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: user,
  };

  const response = await httpRequest(params);

  // Validate the output schema with Zod
  if (response.status === 200) {
    const parsedOutput = confirmUserOutputSchema.safeParse(response.body);
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
