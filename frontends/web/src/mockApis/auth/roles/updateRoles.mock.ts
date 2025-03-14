/**
 * updateRoles.mock.ts
 *
 * Provides a mock version of the `updateRoles` API function for testing purposes.
 * This module re-exports all exports from the original `updateRoles` module and replaces
 * the `updateRoles` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/roles/updateRoles';
import type { UpdateRolesFunction } from 'api-modules/serverApi/auth/roles/updateRoles';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/roles/updateRoles';

/**
 * Mock implementation of the `updateRoles` function.
 *
 * This function wraps the actual `updateRoles` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const updateRoles = fn(actual.updateRoles).mockName('updateRoles');
