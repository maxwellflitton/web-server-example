/**
 * unblockUser.mock.ts
 *
 * Provides a mock version of the `unblockUser` API function for testing purposes.
 * This module re-exports all exports from the original `unblockUser` module and replaces
 * the `unblockUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/unblockUser';
import type { UnblockUserFunction } from 'api-modules/serverApi/auth/users/unblockUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/unblockUser';

/**
 * Mock implementation of the `unblockUser` function.
 *
 * This function wraps the actual `unblockUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const unblockUser = fn(actual.unblockUser).mockName('unblockUser');