/**
 * resendConfirmationEmail.ts
 *
 * # Overview
 * This module provides functionality to resend a confirmation email via the API endpoint.
 * It targets the `/api/auth/v1/users/resend_confirmation_email` endpoint.
 * A successful resend returns a `200 OK` status.
 */

import { AuthUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when resending a confirmation email.
 *
 * - `email`: The email address of the user to whom the confirmation email should be resent.
 */
const resendConfirmationEmailInputSchema = z.object({
  email: z.string().email(),
});

/**
 * Zod schema for the output data after resending a confirmation email.
 * Returns an empty object on success.
 */
const resendConfirmationEmailOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas
export type ResendConfirmationEmailInputSchema = z.infer<typeof resendConfirmationEmailInputSchema>;
export type ResendConfirmationEmailOutputSchema = z.infer<typeof resendConfirmationEmailOutputSchema>;

/**
 * Type definition for the function that resends a confirmation email.
 */
export type ResendConfirmationEmailFunction = ApiFunction<
  ResendConfirmationEmailInputSchema,
  ResendConfirmationEmailOutputSchema
>;

/**
 * Resends a confirmation email by sending a POST request to the designated endpoint.
 *
 * # Arguments
 * - `params`: An object conforming to `ResendConfirmationEmailInputSchema` containing:
 *   - The email address of the user.
 *
 * # Returns
 * - A `Promise` resolving to `ApiResponse<ResendConfirmationEmailOutputSchema>` if successful,
 *   or an `ErrorResponse` if the operation fails.
 *
 * # Errors
 * - Returns a 500 error if there is an unexpected failure or if the response body validation fails.
 *
 * # Usage
 * ```typescript
 * import { resendConfirmationEmail } from "./resendConfirmationEmail";
 *
 * const params = { email: "user@example.com" };
 *
 * resendConfirmationEmail(params)
 *   .then(response => console.log("Confirmation email resent successfully"))
 *   .catch(error => console.error("Failed to resend confirmation email:", error));
 * ```
 */
export async function resendConfirmationEmail(
  params: ResendConfirmationEmailInputSchema, 
  jwt: string
): Promise<ApiResponse<ResendConfirmationEmailOutputSchema>> {
  const url = new AuthUrl().resendConfirmationEmail;
  const httpParams: HttpRequestParams = {
    url,
    httpMethod: "post",
    args: params,
    jwt: jwt
  };

  const response = await httpRequest(httpParams);

  if (response.status === 200) {
    const parsedOutput = resendConfirmationEmailOutputSchema.safeParse(response.body);
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
