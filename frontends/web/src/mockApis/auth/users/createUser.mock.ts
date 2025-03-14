/**
 * createUser.mock.ts
 *
 * Provides a mock version of the `createUser` API function for testing purposes.
 * This module re-exports all exports from the original `createUser` module and replaces
 * the `createUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/createUser';
import type { CreateUserFunction } from 'api-modules/serverApi/auth/users/createUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/createUser';

/**
 * Mock implementation of the `createUser` function.
 *
 * This function wraps the actual `createUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const createUser = fn(actual.createUser).mockName('createUser');
