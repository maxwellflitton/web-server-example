 /**
 * login.mock.ts
 *
 * Provides a mock version of the `login` API function for testing purposes.
 * This module re-exports all exports from the original `login` module and replaces
 * the `login` function with a mock implementation using Storybook's `fn`.
 */

 import { fn } from '@storybook/test';
 import * as actual from 'api-modules/serverApi/auth/auth/login';
 import type { LoginFunction } from 'api-modules/serverApi/auth/auth/login';
 
 // Re-export everything from the original module
 export * from 'api-modules/serverApi/auth/auth/login';
 
 /**
  * Mock implementation of the `login` function.
  *
  * This function wraps the actual `login` implementation,
  * allowing tests to simulate API behavior without making real HTTP requests.
  */
 export const loginUser = fn(actual.loginUser).mockName('login');
 