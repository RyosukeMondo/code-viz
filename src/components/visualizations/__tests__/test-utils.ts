/**
 * Shared test utilities for Treemap component tests
 */

import { vi } from 'vitest';

/**
 * Creates a complete mock ECharts instance with all required methods
 */
export function createMockEChartsInstance() {
  const mockSetOption = vi.fn();
  const mockOn = vi.fn();
  const mockOff = vi.fn();
  const mockResize = vi.fn();
  const mockDispose = vi.fn();

  const mockChartInstance = {
    setOption: mockSetOption,
    on: mockOn,
    off: mockOff,
    resize: mockResize,
    dispose: mockDispose,
  };

  return {
    mockChartInstance,
    mockSetOption,
    mockOn,
    mockOff,
    mockResize,
    mockDispose,
  };
}

/**
 * Sets up ECharts mocks and window event listeners
 * Returns cleanup function and mock objects
 */
export function setupEChartsMocks() {
  const mocks = createMockEChartsInstance();

  // Mock window event listeners
  const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
  const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

  const cleanup = () => {
    vi.clearAllMocks();
  };

  return {
    ...mocks,
    addEventListenerSpy,
    removeEventListenerSpy,
    cleanup,
  };
}

/**
 * Helper to extract event handler from mockOn calls
 */
export function getEventHandler(mockOn: any, eventName: string) {
  const call = mockOn.mock.calls.find((c: any) => c[0] === eventName);
  return call?.[1];
}
