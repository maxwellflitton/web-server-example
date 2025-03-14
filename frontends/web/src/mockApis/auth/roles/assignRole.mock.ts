/**
 * assignRole.mock.ts
 *
 * Provides a mock version of the `assignRole` API function for testing purposes.
 * This module re-exports all exports from the original `assignRole` module and replaces
 * the `assignRole` function with a mock implementation using Storybook's `fn`.
 */

import { fn } from '@storybook/test';
import * as actual from 'api-modules/serverApi/auth/roles/assignRole';
import type { AssignRoleFunction } from 'api-modules/serverApi/auth/roles/assignRole';

// Re-export everything from the original module
export * from 'api-modules/serverApi/auth/roles/assignRole';

/**
 * Mock implementation of the `assignRole` function.
 *
 * This function wraps the actual `assignRole` implementation,
 * allowing tests to simulate API behavior without making real HTTP requests.
 */
export const assignRole = fn(actual.assignRole).mockName('assignRole');
