import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

/**
 * Combines and merges multiple class names into a single string,
 * applying Tailwind Merge to handle conflicting classes gracefully.
 *
 * # Arguments
 * - `...inputs`: A list of class values (strings, arrays, conditionals) to be merged.
 *
 * # Returns
 * - A string containing the merged, de-duplicated class names.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
