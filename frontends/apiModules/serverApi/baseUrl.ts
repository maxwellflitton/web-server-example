/**
 * Configures the URLs for the V0 API.
 *
 * # Overview
 * - Derives the base URL for API requests from the environment variable `BASE_BACKEND_URL`.
 *
 * # Fields
 * - `base`: The root URL for API calls.
 *
 * # Errors
 * - Throws an error if `BASE_BACKEND_URL` is not defined or is empty.
 */
export class VOneUrl {

    public base: string;

    /**
     * Instantiates the VOneUrl class and initializes the base URL.
     *
     * # Behavior
     * - Sets the `base` property by invoking the `defineRoot()` method.
     *
     * # Errors
     * - Propagates errors thrown by `defineRoot()` if the environment variable is missing or empty.
     */
    constructor() {
        this.base = this.defineRoot();
    }

    /**
     * Determines the root URL based on the environment configuration.
     *
     * # Returns
     * - The root URL as defined by the `BASE_BACKEND_URL` environment variable.
     *
     * # Errors
     * - Throws an `Error` if `BASE_BACKEND_URL` is not defined or is an empty string.
     */
    public defineRoot() {
        if (typeof process.env.BASE_BACKEND_URL === "string" && process.env.BASE_BACKEND_URL !== "") {
            return process.env.BASE_BACKEND_URL;
        } else {
            return "http://0.0.0.0:8001";
        }
    }
}
