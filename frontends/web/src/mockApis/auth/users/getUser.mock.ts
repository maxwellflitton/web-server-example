 /**
 * getUser.mock.ts
 *
 * Provides a mock version of the `getUser` API function for testing purposes.
 * This module re-exports all exports from the original `getUser` module and replaces
 * the `getUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/getUser';
import type { GetUserFunction } from 'api-modules/serverApi/auth/users/getUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/getUser';

/**
 * Mock implementation of the `getUser` function.
 *
 * This function wraps the actual `getUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const getUser = fn(actual.getUser).mockName('getUser');
