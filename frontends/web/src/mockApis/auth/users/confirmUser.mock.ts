/**
 * confirmUser.mock.ts
 *
 * Provides a mock version of the `confirmUser` API function for testing purposes.
 * This module re-exports all exports from the original `confirmUser` module and replaces
 * the `confirmUser` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/confirmUser';
import type { ConfirmUserFunction } from 'api-modules/serverApi/auth/users/confirmUser';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/confirmUser';

/**
 * Mock implementation of the `confirmUser` function.
 *
 * This function wraps the actual `confirmUser` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const confirmUser = fn(actual.confirmUser).mockName('confirmUser');
