/**
 * httpRequest.ts
 *
 * Provides a helper function to execute HTTP requests using Axios, with optional JWT
 * and/or custom headers. Returns structured responses consistent with `HttpResponse` type.
 */

import axios, { AxiosError, AxiosRequestConfig, AxiosResponse } from "axios";
import * as apiTypes from "./apiTypes";

/**
 * Defines the arguments to the `httpRequest()` method.
 */
export type HttpRequestParams = {
  url: string;
  httpMethod: apiTypes.HttpMethod;
  args?: any;
  jwt?: string;
  customHeaders?: Record<string, string>;
};

/**
 * Executes an HTTP request using Axios.
 *
 * # Arguments
 * An object containing the HTTP request configuration:
 *   - `url`: The endpoint URL to send the request to.
 *   - `httpMethod`: The HTTP method (e.g., "get", "post", etc.).
 *   - `args`: (Optional) The request payload to be included in the body.
 *   - `jwt`: (Optional) A JSON Web Token for authorization, if required.
 *   - `customHeaders`: (Optional) An object containing additional headers to merge with defaults.
 *
 * # Returns 
 * A `Promise<HttpResponse>` that resolves to an object with:
 *   - `status`: The HTTP status code.
 *   - `body`: The response data, which can be:
 *     1. The expected data on success (for status codes 200 or 201).
 *     2. A simple error message string in rare cases.
 *
 * # Errors 
 * Will return an error-like `HttpResponse` with appropriate status & message if a request fails.
 * 
 * # Notes
 * Our Nanoservice error in the backend is built like `HttpResponse::build(status_code).json(self.message.clone())`.
 * This means its only the message field of the Nanoservice error which is serialized to JSON returning only a string.
 * This means from Axios's pov response.data is a raw string - not an object with "message" and "status" fields.
 */
export async function httpRequest({
  url,
  httpMethod,
  args,
  jwt,
  customHeaders,
}: HttpRequestParams): Promise<apiTypes.HttpResponse> {
  // Prepare Axios config
  const config: AxiosRequestConfig = {
    url,
    method: httpMethod,
    headers: {
      "Content-Type": "application/json",
      ...(jwt ? { token: jwt } : {}),
      ...(customHeaders ?? {}),
    },
    ...(args ? { data: args } : {}),
  };

  try {
    const response: AxiosResponse = await axios(config);

    // Check for standard success codes
    if (response.status === 200 || response.status === 201) {
      let parsedBody: unknown = response.data;

      if (!parsedBody || (typeof parsedBody === "string" && parsedBody.trim().length === 0)) {
        parsedBody = {};
      }

      return { status: response.status, body: parsedBody };
    }

    // Handle unexpected status codes (rare with Axios if it doesn't throw for errors)
    return {
      status: response.status,
      body: response.data || `Unexpected response code: ${response.status}`,
    };
  } catch (error) {
    if (axios.isAxiosError(error)) {
      const axiosError = error as AxiosError<string>;

      if (axiosError.response) {
        // Error responses from the backend with a status code
        return {
          status: axiosError.response.status,
          body: axiosError.response.data || "Unknown Error",
        };
      } else if (axiosError.request) {
        // Network errors or server unreachable
        return {
          status: 0,
          body: "Network error or server unreachable",
        };
      } else {
        // JSON parsing errors or unexpected Axios errors
        return {
          status: 500,
          body: "Invalid JSON in response or unexpected error",
        };
      }
    }

    // Handle unexpected non-Axios errors
    console.warn("Unexpected error:", error);
    return { status: 500, body: "Unexpected error occurred" };
  }
}
