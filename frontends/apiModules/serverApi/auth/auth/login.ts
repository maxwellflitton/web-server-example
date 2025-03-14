/**
 * loginUser.ts
 *
 * Provides functionality to log in a user via the `/login` endpoint.
 * Uses Basic Auth for `email` and `password` in the request header,
 * along with an optional `role` in the request body.
 */

import { AuthUrl } from "./url";
import { ApiResponse, ApiFunction, ErrorResponse } from "../../helpers/apiTypes";
import { httpRequest, HttpRequestParams } from "../../helpers/httpRequest";
import { z } from "zod";
import { Buffer } from 'buffer';

/**
 * Zod schema for the input data when logging in.
 * - `email`: Validates email using Zod's built-in email validator.
 * - `password`: A simple string.
 * - `role`: A string representing the user role.
 */
const loginInputSchema = z.object({
  email: z.string().email(),
  password: z.string(),
  role: z.enum([
    "Super Admin",
    "Admin",
    "Worker"
  ]),
});

/**
 * Zod schema for the output data after a successful login.
 * - `token`: A string representing the returned JWT or session token.
 */
const loginOutputSchema = z.object({
    token: z.string(),
    role: z.enum([
      "Super Admin",
      "Admin",
      "Worker"
    ]),
}).strict();

/** Types inferred from the Zod schemas */
export type LoginInputSchema = z.infer<typeof loginInputSchema>;
export type LoginOutputSchema = z.infer<typeof loginOutputSchema>;

/** Type definition for the function that handles user login. */
export type LoginFunction = ApiFunction<LoginInputSchema, LoginOutputSchema>;

/**
 * Logs in a user by sending a POST request with Basic Auth.
 *
 * @param user - An object conforming to `LoginInputSchema`, containing the user's credentials and role.
 * @returns A `Promise` resolving to `ApiResponse<LoginOutputSchema>` if login is successful, or `ErrorResponse` on error.
 *
 */
export async function loginUser(
  user: LoginInputSchema
): Promise<ApiResponse<LoginOutputSchema>> {
  // Basic Auth credentials
  const credentials = `${user.email}:${user.password}`;
  const encodedCredentials = Buffer.from(credentials).toString("base64");
  const authHeaderValue = `Basic ${encodedCredentials}`;

  // Prepare request params
  const params: HttpRequestParams = {
    url: new AuthUrl().loginUser,
    httpMethod: "post",
    args: { role: user.role }, // The request body
    customHeaders: {
      Authorization: authHeaderValue,
    },
  };

  // Execute the request
  const response = await httpRequest(params);

  // Expect a 200 status for successful login
  if (response.status === 200) {
    const parsedOutput = loginOutputSchema.safeParse(response.body);
    if (!parsedOutput.success) {
      return {
        status: 500,
        body: `Return body validation error - ${parsedOutput.error.message}`,
      } as ErrorResponse;
    }
    return { status: response.status, body: parsedOutput.data };
  }

  // Set the jwt and return the error or unexpected status as is
  return response as ErrorResponse;
}