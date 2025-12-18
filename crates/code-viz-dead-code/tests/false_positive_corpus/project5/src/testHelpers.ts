// Test helper utilities - only used in tests, not in production code
export function createMockUser() {
  return { id: '123', name: 'Test User' };
}

export function createMockData() {
  return { value: 42, label: 'test' };
}

export function assertDeepEqual(a: any, b: any) {
  if (JSON.stringify(a) !== JSON.stringify(b)) {
    throw new Error('Not equal');
  }
}
