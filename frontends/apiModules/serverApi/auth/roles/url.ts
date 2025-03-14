/**
 * AuthUrl.ts
 * 
 * # Overview
 * - Provides URL endpoints for role-related API requests.
 * - Extends the VOneUrl class to include base URL configuration for version 1 of the authentication API.
 */

import { VOneUrl } from "../../baseUrl";

/**
 * Configures the URL endpoints for the Roles API.
 *
 * # Overview
 * - Constructs the base URL for user-related API endpoints by appending `/v1/auth/roles` to the root URL.
 *
 * # Fields
 * - `base`: The base URL for the User API.
 * - `loginUser`: The URL endpoint for logging in a user.
 */
export class RoleUrl extends VOneUrl {

    public base: string;
    public assignRole: string;
    public removeRole: string;
    public updateUserRoles: string;

    /**
     * Instantiates the RoleUrl class and configures the role-auth-related endpoints.
     *
     * # Behavior
     * - Calls the parent `VOneUrl` constructor.
     * - Appends `/v1/auth/auth` to the base URL obtained from `defineRoot()`.
     * - Constructs the endpoint for creating a new role by appending "login" to the base URL.
     */
    constructor() {
        super();
        this.base = this.defineRoot() + "/api/auth/v1/roles";
        this.assignRole = this.base + "/assign_role";
        this.removeRole = this.base + "/remove_role";
        this.updateUserRoles = this.base + "/update"
    }
}
