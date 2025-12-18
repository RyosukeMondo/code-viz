/**
 * Utility helper functions.
 * Some are used, some are dead.
 */

/**
 * Exported helper that is only imported by dead.ts
 * Since dead.ts is never used, this is transitively dead
 * EXPECTED: DEAD - confidence ~70 (exported but only used by dead code)
 */
export function helperForDeadCode(n: number): boolean {
  return n % 2 === 0;
}

/**
 * Completely unused exported utility
 * EXPECTED: DEAD - confidence ~70 (exported, never imported)
 */
export function unusedUtility(str: string): string {
  return str.trim();
}

/**
 * Dead unexported utility
 * EXPECTED: DEAD - confidence 100 (unexported, unused)
 */
function privateUnusedHelper(): void {
  console.log("Never executed");
}
