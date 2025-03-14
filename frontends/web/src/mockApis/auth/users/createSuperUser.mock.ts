/**
 * createSuperUser.mock.ts
 *
 * Provides a mock version of the `createSuperUser` API function for testing purposes.
 * This module re-exports all exports from the original `createUser` module and replaces
 * the `createSuperUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import  * as actual from 'api-modules/serverApi/auth/users/createSuperUser';
import type { CreateSuperUserFunction } from 'api-modules/serverApi/auth/users/createSuperUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/createSuperUser';

/**
 * Mock implementation of the `createSuperUser` function.
 *
 * This function wraps the actual `createSuperUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const createSuperUser = fn(actual.createSuperUser).mockName('createSuperUser');
