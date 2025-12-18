import { calculate, process } from '../src/index.ts';
import { createMockUser, createMockData, assertDeepEqual } from '../src/testHelpers.ts';

// Test that uses helpers
export function testCalculate() {
  const result = calculate(1, 2);
  assertDeepEqual(result, 3);
}

export function testProcess() {
  const data = createMockData();
  const result = process(' hello ');
  assertDeepEqual(result, 'hello');
}

testCalculate();
testProcess();
