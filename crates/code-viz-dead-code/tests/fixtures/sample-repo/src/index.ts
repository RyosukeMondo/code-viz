/**
 * Index file that re-exports from other modules.
 * This file itself is never imported (it's not used as a library).
 * EXPECTED: All re-exports here should be DEAD (exported but never imported from index)
 */

// Re-export everything from used module
export * from './used';

// Re-export specific items from internal
export { publicApi } from './internal';

// Re-export dead module (transitively dead)
export * from './dead';

/**
 * Function defined and exported in index
 * EXPECTED: DEAD - confidence ~70 (exported from index, but index is never imported)
 */
export function indexFunction(): void {
  console.log("Index function never called");
}
