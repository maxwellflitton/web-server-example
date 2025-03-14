/**
 * deleteUser.mock.ts
 *
 * Provides a mock version of the `deleteUser` API function for testing purposes.
 * This module re-exports all exports from the original `deleteUser` module and replaces
 * the `deleteUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/deleteUser';
import type { DeleteUserFunction } from 'api-modules/serverApi/auth/users/deleteUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/deleteUser';

/**
 * Mock implementation of the `deleteUser` function.
 *
 * This function wraps the actual `deleteUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const deleteUser = fn(actual.deleteUser).mockName('deleteUser');