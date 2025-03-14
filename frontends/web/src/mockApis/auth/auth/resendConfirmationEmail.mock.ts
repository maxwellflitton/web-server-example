/**
 * resendConfirmationEmail.mock.ts
 *
 * Provides a mock version of the `resendConfirmationEmail` API function for testing purposes.
 * This module re-exports all exports from the original `resendConfirmationEmail` module and replaces
 * the `resendConfirmationEmail` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/auth/resendConfirmationEmail';
import type { ResendConfirmationEmailFunction } from 'api-modules/serverApi/auth/auth/resendConfirmationEmail';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/auth/resendConfirmationEmail';

/**
 * Mock implementation of the `resendConfirmationEmail` function.
 *
 * This function wraps the actual `resendConfirmationEmail` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const resendConfirmationEmail = fn(actual.resendConfirmationEmail).mockName('resendConfirmationEmail');
