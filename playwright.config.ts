import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for Tauri E2E testing
 *
 * This config is tailored for testing desktop applications built with Tauri.
 * It assumes the Tauri app is built and ready to test.
 */
export default defineConfig({
  testDir: './tests/e2e',

  // Run tests in files sequentially
  fullyParallel: false,

  // Fail the build on CI if you accidentally left test.only in the source code
  forbidOnly: !!process.env.CI,

  // Retry on CI only
  retries: process.env.CI ? 2 : 0,

  // Reporter to use
  reporter: [
    ['html'],
    ['list'],
  ],

  // Shared settings for all the projects below
  use: {
    // Base URL for web-based tests (not used for Tauri, but kept for consistency)
    // baseURL: 'http://localhost:5173',

    // Collect trace when retrying the failed test
    trace: 'on-first-retry',

    // Screenshot only on failure
    screenshot: 'only-on-failure',

    // Video on failure
    video: 'retain-on-failure',
  },

  // Configure projects for major desktop platforms
  projects: [
    {
      name: 'Desktop',
      use: {
        ...devices['Desktop Chrome'],
        // Tauri apps run as native desktop applications
        // We'll use playwright to control the app via automation
      },
    },
  ],

  // Timeout settings
  timeout: 30000, // 30 seconds for each test
  expect: {
    timeout: 10000, // 10 seconds for assertions
  },
});
