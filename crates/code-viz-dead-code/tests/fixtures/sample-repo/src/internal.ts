/**
 * This file demonstrates internal (non-exported) functions.
 * Some are used internally, some are dead code.
 */

/**
 * Unexported function used by another function in this file
 * EXPECTED: LIVE - used internally by publicApi
 */
function internalUsedHelper(data: string): number {
  return data.length;
}

/**
 * Exported function that uses internal helper
 * EXPECTED: LIVE - imported in main.ts
 */
export function publicApi(input: string): number {
  return internalUsedHelper(input) * 2;
}

/**
 * Unexported function that is NEVER used anywhere
 * EXPECTED: DEAD - confidence 100 (unexported, unused)
 */
function completelyUnused(x: number, y: number): number {
  return x + y + 42;
}

/**
 * Another dead unexported function
 * EXPECTED: DEAD - confidence 100 (unexported, unused)
 */
function anotherUnusedFunction(): void {
  console.log("Never called");
}

/**
 * Exported but only used in dead.ts (which is itself dead)
 * EXPECTED: DEAD - confidence ~70 (exported, but only used by dead code)
 */
export function onlyUsedByDeadCode(): string {
  return "I am transitively dead";
}
