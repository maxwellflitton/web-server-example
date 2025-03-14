/**
 * requestPasswordReset.mock.ts
 *
 * Provides a mock version of the `requestPasswordReset` API function for testing purposes.
 * This module re-exports all exports from the original `requestPasswordReset` module and replaces
 * the `requestPasswordReset` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/auth/requestPasswordReset';
import type { RequestPasswordResetFunction } from 'api-modules/serverApi/auth/auth/requestPasswordReset';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/auth/requestPasswordReset';

/**
 * Mock implementation of the `requestPasswordReset` function.
 *
 * This function wraps the actual `requestPasswordReset` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const requestPasswordReset = fn(actual.requestPasswordReset).mockName('requestPasswordReset');
