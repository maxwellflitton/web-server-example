/**
 * Delays execution by pausing for a specified duration.
 *
 * # Arguments
 * - `timeMs`: The number of milliseconds to wait before the promise resolves.
 *
 * # Returns
 * - A `Promise<void>` that resolves after the specified delay.
 */
export function sleep(timeMs: number) {
	return new Promise<void>((resolve) => setTimeout(resolve, timeMs))
}