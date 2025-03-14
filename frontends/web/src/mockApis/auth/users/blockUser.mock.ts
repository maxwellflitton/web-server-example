/**
 * blockUser.mock.ts
 *
 * Provides a mock version of the `blockUser` API function for testing purposes.
 * This module re-exports all exports from the original `blockUser` module and replaces
 * the `blockUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/blockUser';
import type { BlockUserFunction } from 'api-modules/serverApi/auth/users/blockUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/blockUser';

/**
 * Mock implementation of the `blockUser` function.
 *
 * This function wraps the actual `blockUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const blockUser = fn(actual.blockUser).mockName('blockUser');
