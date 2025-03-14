/**
 * resetPassword.mock.ts
 *
 * Provides a mock version of the `resetPassword` API function for testing purposes.
 * This module re-exports all exports from the original `resetPassword` module and replaces
 * the `resetPassword` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/users/resetPassword';
import type { ResetPasswordFunction } from 'api-modules/serverApi/auth/users/resetPassword';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/users/resetPassword';

/**
 * Mock implementation of the `resetPassword` function.
 *
 * This function wraps the actual `resetPassword` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const resetPassword = fn(actual.resetPassword).mockName('resetPassword');
