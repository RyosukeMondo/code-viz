/**
 * Part of a circular import test.
 * This file imports from circular-b.ts, which imports from this file.
 * Both are unused, so both should be dead despite the circular dependency.
 */

import { functionB } from './circular-b';

/**
 * Exported function that creates a circular dependency
 * EXPECTED: DEAD - confidence ~70 (exported but part of unused circular dependency)
 */
export function functionA(): string {
  return 'A calls B: ' + functionB();
}

/**
 * Unexported helper in circular module
 * EXPECTED: DEAD - confidence 100 (unexported, part of dead circular dependency)
 */
function helperA(): number {
  return 1;
}
