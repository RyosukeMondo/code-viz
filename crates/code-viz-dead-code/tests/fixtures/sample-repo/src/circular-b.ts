/**
 * Part of a circular import test.
 * This file imports from circular-a.ts, which imports from this file.
 * Both are unused, so both should be dead despite the circular dependency.
 */

import { functionA } from './circular-a';

/**
 * Exported function that creates a circular dependency
 * EXPECTED: DEAD - confidence ~70 (exported but part of unused circular dependency)
 */
export function functionB(): string {
  // Note: We can't actually call functionA here or we'd get infinite recursion
  // This demonstrates that circular imports exist structurally even if not executed
  return 'B (could call A)';
}

/**
 * Unexported helper in circular module
 * EXPECTED: DEAD - confidence 100 (unexported, part of dead circular dependency)
 */
function helperB(): number {
  return 2;
}
