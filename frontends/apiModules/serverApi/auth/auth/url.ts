/**
 * AuthUrl.ts
 * 
 * # Overview
 * - Provides URL endpoints for user-related API requests.
 * - Extends the VOneUrl class to include base URL configuration for version 1 of the authentication API.
 */

import { VOneUrl } from "../../baseUrl";

/**
 * Configures the URL endpoints for the User API.
 *
 * # Overview
 * - Constructs the base URL for user-related API endpoints by appending `/v1/auth/auth` to the root URL.
 * - Provides endpoints for creating a standard user and a super user.
 *
 * # Fields
 * - `base`: The base URL for the User API.
 * - `loginUser`: The URL endpoint for logging in a user.
 */
export class AuthUrl extends VOneUrl {

    public base: string;
    public loginUser: string;
    public requestPasswordReset: string;
    public resendConfirmationEmail: string;

    /**
     * Instantiates the UserUrl class and configures the user-auth-related endpoints.
     *
     * # Behavior
     * - Calls the parent `VOneUrl` constructor.
     * - Appends `/v1/auth/auth` to the base URL obtained from `defineRoot()`.
     * - Constructs the endpoint for creating a new user by appending "login" to the base URL.
     */
    constructor() {
        super();
        this.base = this.defineRoot() + "/api/auth/v1/auth";
        this.loginUser = this.base + "/login";
        this.requestPasswordReset = this.base + "/request_password_reset";
        this.resendConfirmationEmail = this.base + "/resend_confirmation_email";
    }
}
