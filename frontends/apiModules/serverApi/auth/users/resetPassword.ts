/**
 * resetPassword.ts
 *
 * # Overview
 * This module provides functionality to reset a user's password via the API endpoint.
 * It targets the `/api/auth/v1/users/reset_password` endpoint and ensures that a successful reset
 * returns a `200 OK` status. Robust error handling is included to manage unexpected responses.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when resetting a user's password.
 *
 * - `unique_id`: The user's current uuid. 
 * - `password`: A string representing the new password. 
 */
const resetPasswordInputSchema = z.object({
  unique_id: z.string().uuid(),
  new_password: z.string(),
});

/**
 * Zod schema for the output data after resetting a user's password.
 * Returns an empty object on success.
 */
const resetPasswordOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type ResetPasswordInputSchema = z.infer<typeof resetPasswordInputSchema>;
export type ResetPasswordOutputSchema = z.infer<typeof resetPasswordOutputSchema>;

/**
 * Type definition for the function that resets a user's password.
 */
export type ResetPasswordFunction = ApiFunction<ResetPasswordInputSchema, ResetPasswordOutputSchema>;

/**
 * Resets a user's password by sending a POST request to the password reset endpoint.
 *
 * # Arguments
 * - `data`: An object conforming to `ResetPasswordInputSchema` containing the new password.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<ResetPasswordOutputSchema>` if the password is reset successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `200 OK` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { resetPassword, ResetPasswordInputSchema } from "./resetPassword";
 *
 * const data: ResetPasswordInputSchema = { 
 *   password: "newsecurepassword" 
 * };
 *
 * resetPassword(data)
 *   .then(response => console.log("Password reset successfully:", response))
 *   .catch(error => console.error("Error resetting password:", error));
 * ```
 */
export async function resetPassword(
  data: ResetPasswordInputSchema
): Promise<ApiResponse<ResetPasswordOutputSchema>> {
  const url = new UserUrl().resetPassword;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: data,
  };

  const response = await httpRequest(params);

  if (response.status === 200) {
    const parsedOutput = resetPasswordOutputSchema.safeParse(response.body);
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
