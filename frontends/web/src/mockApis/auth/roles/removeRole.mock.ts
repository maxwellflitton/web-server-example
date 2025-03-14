/**
 * removeRole.mock.ts
 *
 * Provides a mock version of the `removeRole` API function for testing purposes.
 * This module re-exports all exports from the original `removeRole` module and replaces
 * the `removeRole` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/roles/removeRole';
import type { RemoveRoleFunction } from 'api-modules/serverApi/auth/roles/removeRole';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/roles/removeRole';

/**
 * Mock implementation of the `removeRole` function.
 *
 * This function wraps the actual `removeRole` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const removeRole = fn(actual.removeRole).mockName('removeRole');
