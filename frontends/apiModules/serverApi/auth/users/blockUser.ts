/**
 * blockUser.ts
 *
 * # Overview
 * This module provides functionality to interact with the user blocking API endpoint using Axios.
 * It targets the `/block_user` endpoint and ensures that a successful blocking action returns a `200 OK` status.
 * Robust error handling is included to manage unexpected responses.
 */

import { UserUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";

/**
 * Zod schema for the input data when blocking a user.
 *
 * - `user_id`: A number representing the unique identifier of the user to block.
 */
const blockUserInputSchema = z.object({
  user_id: z.number(),
});

/**
 * Zod schema for the output data after a user is blocked.
 * (Currently defined as an empty object; adjust as needed.)
 */
const blockUserOutputSchema = z.object({}).strict();

// Export types inferred from the Zod schemas.
export type BlockUserInputSchema = z.infer<typeof blockUserInputSchema>;
export type BlockUserOutputSchema = z.infer<typeof blockUserOutputSchema>;

/**
 * Type definition for the function that blocks a user.
 */
export type BlockUserFunction = ApiFunction<BlockUserInputSchema, BlockUserOutputSchema>;

/**
 * Blocks a user by sending a POST request to the user block endpoint.
 *
 * # Arguments
 * - `user`: An object conforming to the `BlockUserInputSchema` interface containing the user's blocking details.
 *
 * # Returns
 * - A `Promise` resolving to an `ApiResponse<BlockUserOutputSchema>` if the user is blocked successfully.
 *   Otherwise, it resolves to an `ErrorResponse`.
 *
 * # Errors
 * - Returns an error response if the API call does not return a `200 OK` status or if an unexpected error occurs.
 *
 * # Usage
 * ```typescript
 * import { blockUser, BlockUserInputSchema } from "./path-to-module";
 *
 * const user: BlockUserInputSchema = { 
 *   user_id: 2 
 * };
 *
 * blockUser(user, jwt)
 *   .then(response => console.log("User blocked successfully:", response))
 *   .catch(error => console.error("Error blocking user:", error));
 * ```
 */
export async function blockUser(user: BlockUserInputSchema, jwt: string): Promise<ApiResponse<BlockUserOutputSchema>> {
  const url = new UserUrl().blockUser;
  const params: HttpRequestParams = {
    url: url,
    httpMethod: "post",
    args: user,
    jwt: jwt
  };
  const response = await httpRequest(params);

  // Validate the output schema with Zod
  if (response.status === 200) {
    const parsedOutput = blockUserOutputSchema.safeParse(response.body);
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