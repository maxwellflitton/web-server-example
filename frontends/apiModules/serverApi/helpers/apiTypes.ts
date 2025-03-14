/**
 * A TypeScript file with useful types for HTTP Requests.
 */

/**
 * Defines the possible status codes for HTTP Responses.
 */
type StatusCode = 0 | 200 | 201 | 401 | 404 | 500
type SuccessStatusCode = 200 | 201
type ErrorStatusCode = Exclude<StatusCode, SuccessStatusCode>

/**
 * Defines the possible HTTP methods.
 */
export type HttpMethod = "get" | "post"

/**
 * Defines the possible HTTP and Api method responses.
 */
export type ErrorResponse = { status: ErrorStatusCode; body: string }
export type HttpResponse<Body = any> = { status: SuccessStatusCode; body: Body } | { status: number; body: any }
export type ApiResponse<Body> = { status: SuccessStatusCode; body: Body } | ErrorResponse

/**
 * Defines the possible Api Function types.
 */
export type ApiFunction<Input, Output> = (args: Input) => Promise<ApiResponse<Output>>
export type ApiFunctionNoInput<Output> = () => Promise<ApiResponse<Output>>
