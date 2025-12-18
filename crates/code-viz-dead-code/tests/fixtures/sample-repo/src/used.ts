/**
 * This file contains functions that are actively used in the codebase.
 * All exported functions should be marked as LIVE.
 */

/**
 * Exported function that is imported and used in main.ts
 * EXPECTED: LIVE - used in main.ts
 */
export function activeFunction(x: number): number {
  return x * 2;
}

/**
 * Exported function that is imported and used in tests
 * EXPECTED: LIVE - used in app.test.ts
 */
export function testableFunction(str: string): string {
  return str.toUpperCase();
}

/**
 * Internal helper function used within this module
 * EXPECTED: LIVE - used internally by activeFunction
 */
function internalHelper(n: number): boolean {
  return n > 0;
}

/**
 * Another exported function used by main.ts
 * EXPECTED: LIVE - used in main.ts
 */
export function processData(data: string[]): number {
  return data.filter(s => internalHelper(s.length)).length;
}
