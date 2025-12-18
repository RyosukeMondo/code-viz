/**
 * This file contains dead code - exported but never imported anywhere.
 * All exported functions should be marked as DEAD with high confidence.
 */

/**
 * Exported function that is never imported or used
 * EXPECTED: DEAD - confidence ~70 (exported reduces confidence to 70)
 */
export function unusedExportedFunction(x: number): string {
  return `Result: ${x}`;
}

/**
 * Exported class that is never instantiated
 * EXPECTED: DEAD - confidence ~70 (exported reduces confidence to 70)
 */
export class UnusedClass {
  private value: number;

  constructor(val: number) {
    this.value = val;
  }

  public getValue(): number {
    return this.value;
  }
}

/**
 * Exported async function never called
 * EXPECTED: DEAD - confidence ~70 (exported reduces confidence to 70)
 */
export async function deadAsyncFunction(): Promise<void> {
  console.log("This never runs");
}

/**
 * Default export that is never imported
 * EXPECTED: DEAD - confidence ~70 (exported reduces confidence to 70)
 */
export default function unusedDefault(): boolean {
  return true;
}
