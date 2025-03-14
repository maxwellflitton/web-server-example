/**
 * localStorage.mock.ts
 *
 * Provides a mock version of the `localStorage` utilities (`getJwt`, `setJwt`).
 * This module re-exports everything from the real localStorage file, and wraps the
 * key functions in Storybook's `fn`, so you can mockReturnValue or mockResolvedValue, etc.
 */

import { fn } from "@storybook/test";
// If your real localStorage file is located at `api-modules/auth/localStorage.ts`:
import * as actual from "api-modules/auth/localStorage";

// Re-export everything so you still have access to other exports if needed
export * from "api-modules/auth/localStorage";

// Overwrite the specific functions you want to mock
export const setJwt = fn(actual.setJwt).mockName("setJwt");
export const getJwt = fn(actual.getJwt).mockName("getJwt");
export const setRole = fn(actual.setRole).mockName("setRole");
export const getRole = fn(actual.getRole).mockName("getRole");
