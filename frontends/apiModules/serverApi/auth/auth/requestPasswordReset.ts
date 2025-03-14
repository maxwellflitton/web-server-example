/**
 * requestPasswordReset.ts
 *
 * # Overview
 * This module provides functionality to request a password reset via the API endpoint.
 * It targets the `/api/auth/v1/users/request_password_reset` endpoint and ensures that a successful request
 * returns a `200 OK` status with an empty body. Robust error handling is included to manage unexpected responses.
 */

import { AuthUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when requesting a password reset.
 *
 * - `email`: The user's email address.
 */
const requestPasswordResetInputSchema = z.object({
  email: z.string().email(),
});

/**
 * Zod schema for the output data after requesting a password reset.
 * Returns an empty object on success.
 */
const requestPasswordResetOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type RequestPasswordResetInputSchema = z.infer<typeof requestPasswordResetInputSchema>;
export type RequestPasswordResetOutputSchema = z.infer<typeof requestPasswordResetOutputSchema>;

/**
 * Type definition for the function that requests a password reset.
 */
export type RequestPasswordResetFunction = ApiFunction<RequestPasswordResetInputSchema, RequestPasswordResetOutputSchema>;

/**
 * Requests a password reset for a user by sending a POST request to the password reset request endpoint.
 *
 * # Arguments
 * - `data`: An object conforming to `RequestPasswordResetInputSchema` containing the user's email.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<RequestPasswordResetOutputSchema>` if the request is successful.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `200 OK` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { requestPasswordReset, RequestPasswordResetInputSchema } from "./requestPasswordReset";
 *
 * const data: RequestPasswordResetInputSchema = { 
 *   email: "user@example.com" 
 * };
 *
 * requestPasswordReset(data)
 *   .then(response => console.log("Password reset request sent successfully:", response))
 *   .catch(error => console.error("Error requesting password reset:", error));
 * ```
 */
export async function requestPasswordReset(
  data: RequestPasswordResetInputSchema
): Promise<ApiResponse<RequestPasswordResetOutputSchema>> {
  const url = new AuthUrl().requestPasswordReset;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: data,
  };

  const response = await httpRequest(params);

  if (response.status === 200) {
    const parsedOutput = requestPasswordResetOutputSchema.safeParse(response.body);
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
