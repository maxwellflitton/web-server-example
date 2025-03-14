/**
 * getAllUsers.mock.ts
 *
 * Provides a mock version of the `getAllUsers` API function for testing purposes.
 * This module re-exports all exports from the original `getAllUsers` module and replaces
 * the `getAllUsers` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/getAllUsers';
import type { GetAllUsersFunction } from 'api-modules/serverApi/auth/users/getAllUsers';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/getAllUsers';

/**
 * Mock implementation of the `getAllUsers` function.
 *
 * This function wraps the actual `getAllUsers` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const getAllUsers = fn(actual.getAllUsers).mockName('getAllUsers');
