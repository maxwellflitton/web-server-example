/**
 * unblockUser.ts
 *
 * # Overview
 * This module provides functionality to interact with the user unblocking API endpoint using Axios.
 * It targets the `/unblock_user` endpoint and ensures that a successful unblocking action returns a `200 OK` status.
 * Robust error handling is included to manage unexpected responses.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when unblocking a user.
 *
 * - `user_id`: A number representing the unique identifier of the user to unblock.
 */
const unblockUserInputSchema = z.object({
  user_id: z.number(),
});

/**
 * Zod schema for the output data after a user is unblocked.
 * (Currently defined as an empty object; adjust as needed.)
 */
const unblockUserOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type UnblockUserInputSchema = z.infer<typeof unblockUserInputSchema>;
export type UnblockUserOutputSchema = z.infer<typeof unblockUserOutputSchema>;

/**
 * Type definition for the function that unblocks a user.
 */
export type UnblockUserFunction = ApiFunction<UnblockUserInputSchema, UnblockUserOutputSchema>;

/**
 * Unblocks a user by sending a POST request to the user unblock endpoint.
 *
 * # Arguments
 * - `user`: An object conforming to the `UnblockUserInputSchema` interface containing the user's unblocking details.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<UnblockUserOutputSchema>` if the user is unblocked successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `200 OK` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { unblockUser, UnblockUserInputSchema } from "./path-to-module";
 *
 * const user: UnblockUserInputSchema = { 
 *   user_id: 2 
 * };
 *
 * unblockUser(user, jwt)
 *   .then(response => console.log("User unblocked successfully:", response))
 *   .catch(error => console.error("Error unblocking user:", error));
 * ```
 */
export async function unblockUser(user: UnblockUserInputSchema, jwt: string): Promise<ApiResponse<UnblockUserOutputSchema>> {
  const url = new UserUrl().unblockUser;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: user,
    jwt: jwt
  };
  const response = await httpRequest(params);

  // Validate the output schema with Zod
  if (response.status === 200) {
    const parsedOutput = unblockUserOutputSchema.safeParse(response.body ?? {});
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
