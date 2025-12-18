/**
 * Test file for the application.
 * This file should be detected as an entry point (test files are entry points).
 * Functions imported here should be marked as LIVE.
 */

import { testableFunction } from '../src/used';

/**
 * Test function - entry point for tests
 * EXPECTED: LIVE - test functions are entry points
 */
function testTestableFunction(): void {
  const result = testableFunction('hello');
  if (result !== 'HELLO') {
    throw new Error('Test failed!');
  }
  console.log('Test passed: testableFunction');
}

/**
 * Another test function
 * EXPECTED: LIVE - test functions are entry points
 */
function testAnotherCase(): void {
  const result = testableFunction('world');
  if (result !== 'WORLD') {
    throw new Error('Test failed!');
  }
  console.log('Test passed: testAnotherCase');
}

/**
 * Unused test helper (dead code even in test files)
 * EXPECTED: DEAD - confidence 100 (unexported, unused, even though in test file)
 */
function unusedTestHelper(): string {
  return "never called";
}

// Run tests
testTestableFunction();
testAnotherCase();
