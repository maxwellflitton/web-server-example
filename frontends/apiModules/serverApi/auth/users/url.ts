/**
 * UserUrl.ts
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
 * - Constructs the base URL for user-related API endpoints by appending `/v1/auth/users` to the root URL.
 * - Provides endpoints for creating a standard user and a super user.
 *
 * # Fields
 * - `base`: The base URL for the User API.
 * - `createUser`: The URL endpoint for creating a new user.
 * - `createSuperUser`: The URL endpoint for creating a super user.
 */
export class UserUrl extends VOneUrl {

    public base: string;
    public createUser: string;
    public createSuperUser: string;
    public confirmUser: string;
    public blockUser: string;
    public unblockUser: string;
    public getUserByUuid: string;
    public getUserById: string;
    public getUserByEmail: string;
    public getUserByJwt: string;
    public getAllUsers: string;
    public deleteUser: string;
    public resetPassword: string;

    /**
     * Instantiates the UserUrl class and configures the user-related endpoints.
     *
     * # Behavior
     * - Calls the parent `VOneUrl` constructor.
     * - Appends `/v1/auth/users` to the base URL obtained from `defineRoot()`.
     * - Constructs the endpoint for creating a new user by appending "create" to the base URL.
     * - Constructs the endpoint for creating a super user by appending "super-admin/create" to the base URL.
     */
    constructor() {
        super();
        this.base = this.defineRoot() + "/api/auth/v1/users";
        this.createUser = this.base + "/create";
        this.createSuperUser = this.base + "/create/superadmin";
        this.confirmUser = this.base + "/confirm";
        this.blockUser = this.base + "/block";
        this.unblockUser = this.base + "/unblock";
        this.getUserById = this.base + "/get-by-id/";
        this.getUserByEmail = this.base + "/get-by-email/";
        this.getUserByJwt = this.base + "/get-by-jwt";
        this.getUserByUuid = this.base + "/get-by-uuid/";
        this.getAllUsers = this.base + "/get-all";
        this.deleteUser = this.base + "/delete";
        this.resetPassword = this.base + "/reset-password";
    }

    constructGetUserById(id: number): string {
        return this.getUserById + id;
    }

    constructGetUserByEmail(email: string): string {
        return this.getUserByEmail + email;
    }

    constructGetUserByUuid(uuid: string): string {
        return this.getUserByUuid + uuid;
    }
}
