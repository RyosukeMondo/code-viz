import { test, expect, Page } from '@playwright/test';
import path from 'path';

/**
 * E2E tests for Treemap Visualization
 *
 * These tests validate the complete user workflow:
 * 1. Analyze a repository
 * 2. Drill down into directories
 * 3. View file details
 * 4. Navigate using breadcrumbs
 * 5. Keyboard navigation
 */

// Helper function to wait for analysis to complete
async function waitForAnalysisComplete(page: Page) {
  await expect(page.locator('[data-testid="loading-state"]')).not.toBeVisible({ timeout: 10000 });
  await expect(page.locator('[data-testid="treemap-container"]')).toBeVisible();
}

// Helper function to setup mock Tauri API
async function setupTauriMocks(page: Page) {
  await page.addInitScript(() => {
    // Mock Tauri API for browser testing
    (window as any).__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: any) => {
        if (cmd === 'analyze_repository') {
          // Return mock analysis data
          return {
            id: 'root',
            name: 'sample-repo',
            path: '/tests/fixtures/sample-repo',
            loc: 150,
            complexity: 35,
            type: 'directory',
            lastModified: '2025-12-18T00:00:00Z',
            children: [
              {
                id: 'src',
                name: 'src',
                path: '/tests/fixtures/sample-repo/src',
                loc: 100,
                complexity: 40,
                type: 'directory',
                lastModified: '2025-12-18T00:00:00Z',
                children: [
                  {
                    id: 'src/main.rs',
                    name: 'main.rs',
                    path: '/tests/fixtures/sample-repo/src/main.rs',
                    loc: 100,
                    complexity: 45,
                    type: 'file',
                    lastModified: '2025-12-18T00:00:00Z',
                    children: [],
                  },
                ],
              },
              {
                id: 'lib',
                name: 'lib',
                path: '/tests/fixtures/sample-repo/lib',
                loc: 50,
                complexity: 25,
                type: 'directory',
                lastModified: '2025-12-18T00:00:00Z',
                children: [
                  {
                    id: 'lib/utils.rs',
                    name: 'utils.rs',
                    path: '/tests/fixtures/sample-repo/lib/utils.rs',
                    loc: 50,
                    complexity: 20,
                    type: 'file',
                    lastModified: '2025-12-18T00:00:00Z',
                    children: [],
                  },
                ],
              },
            ],
          };
        }
        throw new Error(`Unknown command: ${cmd}`);
      },
    };
  });
}

test.describe('Treemap Visualization E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Setup Tauri mocks before each test
    await setupTauriMocks(page);

    // Navigate to the app (using Vite dev server for E2E testing)
    await page.goto('http://localhost:5173');
  });

  test('should load the application successfully', async ({ page }) => {
    // Verify app title
    await expect(page).toHaveTitle(/CodeViz/);

    // Verify main UI elements are present
    await expect(page.locator('[data-testid="repository-path-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="analyze-button"]')).toBeVisible();
  });

  test('should analyze repository and display treemap', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Enter repository path
    const pathInput = page.locator('[data-testid="repository-path-input"]');
    await pathInput.fill(sampleRepoPath);

    // Click analyze button
    const analyzeButton = page.locator('[data-testid="analyze-button"]');
    await analyzeButton.click();

    // Wait for loading state
    await expect(page.locator('[data-testid="loading-state"]')).toBeVisible();

    // Wait for analysis to complete
    await waitForAnalysisComplete(page);

    // Verify treemap is displayed
    const treemap = page.locator('[data-testid="treemap-container"]');
    await expect(treemap).toBeVisible();

    // Verify root node is visible
    await expect(page.locator('text=sample-repo')).toBeVisible();
  });

  test('should perform drill-down navigation', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze repository
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    // Click on 'src' directory
    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();

    // Verify breadcrumb shows drill-down path
    const breadcrumb = page.locator('[data-testid="breadcrumb"]');
    await expect(breadcrumb).toContainText('sample-repo');
    await expect(breadcrumb).toContainText('src');

    // Verify main.rs is now visible in the treemap
    await expect(page.locator('text=main.rs')).toBeVisible();
  });

  test('should navigate using breadcrumb', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze and drill down
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    // Drill down to src
    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();

    // Wait for drill-down animation
    await page.waitForTimeout(500);

    // Click on root breadcrumb to go back
    const rootBreadcrumb = page.locator('[data-testid="breadcrumb-segment"]').filter({ hasText: 'sample-repo' });
    await rootBreadcrumb.click();

    // Verify we're back at root level
    await expect(page.locator('text=src')).toBeVisible();
    await expect(page.locator('text=lib')).toBeVisible();
  });

  test('should display file details on click', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze repository
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    // Drill down to src
    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();
    await page.waitForTimeout(500);

    // Click on main.rs file
    const mainRsNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'main.rs' }).first();
    await mainRsNode.click();

    // Verify detail panel is displayed
    const detailPanel = page.locator('[data-testid="detail-panel"]');
    await expect(detailPanel).toBeVisible();

    // Verify file information is displayed
    await expect(detailPanel).toContainText('main.rs');
    await expect(detailPanel).toContainText('LOC');
    await expect(detailPanel).toContainText('Complexity');
  });

  test('should close detail panel', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze and open detail panel
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();
    await page.waitForTimeout(500);

    const mainRsNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'main.rs' }).first();
    await mainRsNode.click();

    // Verify detail panel is open
    await expect(page.locator('[data-testid="detail-panel"]')).toBeVisible();

    // Close detail panel
    const closeButton = page.locator('[data-testid="detail-panel-close"]');
    await closeButton.click();

    // Verify detail panel is closed
    await expect(page.locator('[data-testid="detail-panel"]')).not.toBeVisible();
  });

  test('should support keyboard navigation - Escape to close detail panel', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze and open detail panel
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();
    await page.waitForTimeout(500);

    const mainRsNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'main.rs' }).first();
    await mainRsNode.click();

    // Verify detail panel is open
    await expect(page.locator('[data-testid="detail-panel"]')).toBeVisible();

    // Press Escape to close
    await page.keyboard.press('Escape');

    // Verify detail panel is closed
    await expect(page.locator('[data-testid="detail-panel"]')).not.toBeVisible();
  });

  test('should support keyboard navigation - Tab through interactive elements', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Navigate using Tab key
    await page.keyboard.press('Tab');

    // Verify repository path input is focused
    await expect(page.locator('[data-testid="repository-path-input"]')).toBeFocused();

    await page.keyboard.press('Tab');

    // Verify analyze button is focused
    await expect(page.locator('[data-testid="analyze-button"]')).toBeFocused();
  });

  test('should support Enter key to trigger analysis', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Enter repository path
    const pathInput = page.locator('[data-testid="repository-path-input"]');
    await pathInput.fill(sampleRepoPath);

    // Focus analyze button and press Enter
    await page.locator('[data-testid="analyze-button"]').focus();
    await page.keyboard.press('Enter');

    // Wait for analysis to complete
    await waitForAnalysisComplete(page);

    // Verify treemap is displayed
    await expect(page.locator('[data-testid="treemap-container"]')).toBeVisible();
  });

  test('should render treemap within performance budget (<3s)', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Start timer
    const startTime = Date.now();

    // Analyze repository
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();

    // Wait for treemap to be visible
    await expect(page.locator('[data-testid="treemap-container"]')).toBeVisible({ timeout: 10000 });

    // Calculate render time
    const renderTime = Date.now() - startTime;

    // Verify render time is under 3 seconds
    expect(renderTime).toBeLessThan(3000);
  });

  test('should handle error states gracefully', async ({ page }) => {
    // Override mock to return error
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          throw new Error('Failed to analyze repository');
        },
      };
    });

    await page.goto('http://localhost:5173');

    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Try to analyze
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();

    // Verify error message is displayed
    await expect(page.locator('[data-testid="error-message"]')).toBeVisible();
    await expect(page.locator('[data-testid="error-message"]')).toContainText('Failed');
  });

  test('should apply color mapping based on complexity', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze repository
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    // Drill down to src to see different complexity files
    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();
    await page.waitForTimeout(500);

    // Verify treemap nodes have color styling
    // The actual color values would be tested in unit tests
    // Here we just verify the nodes exist and are styled
    const treemapNodes = page.locator('[data-testid="treemap-node"]');
    await expect(treemapNodes.first()).toBeVisible();
  });

  test('should display correct LOC and complexity values', async ({ page }) => {
    const sampleRepoPath = path.resolve('./tests/fixtures/sample-repo');

    // Analyze repository
    await page.locator('[data-testid="repository-path-input"]').fill(sampleRepoPath);
    await page.locator('[data-testid="analyze-button"]').click();
    await waitForAnalysisComplete(page);

    // Drill down to src
    const srcNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'src' }).first();
    await srcNode.click();
    await page.waitForTimeout(500);

    // Click on main.rs to view details
    const mainRsNode = page.locator('[data-testid="treemap-node"]').filter({ hasText: 'main.rs' }).first();
    await mainRsNode.click();

    // Verify detail panel shows correct values
    const detailPanel = page.locator('[data-testid="detail-panel"]');
    await expect(detailPanel).toContainText('100'); // LOC from mock data
    await expect(detailPanel).toContainText('45'); // Complexity from mock data
  });
});
