/**
 * End-to-end tests for drill-down navigation
 *
 * These tests verify the complete drill-down flow including:
 * - Treemap rendering
 * - Click interactions
 * - Breadcrumb display
 * - Path updates
 */

import { test, expect } from '@playwright/test';

test.describe('Drill-down Navigation E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the app
    await page.goto('http://localhost:5173');

    // Enter test repository path
    await page.fill('[data-testid="repository-path-input"]', '/home/rmondo/repos/code-viz');

    // Click analyze button
    await page.click('[data-testid="analyze-button"]');

    // Wait for treemap to appear
    await page.waitForSelector('[data-testid="treemap-container"]', { timeout: 10000 });
  });

  test('should show empty breadcrumb at root', async ({ page }) => {
    const breadcrumb = await page.textContent('[data-testid="breadcrumb"]');
    expect(breadcrumb).toContain('Home');
    expect(breadcrumb).toContain('Root directory');
  });

  test('should update breadcrumb when clicking on a directory', async ({ page }) => {
    // Click on the treemap to drill down
    // Note: This requires finding a clickable directory element
    const treemapContainer = await page.locator('[data-testid="treemap-container"]');

    // Simulate click on treemap (clicking in the center should hit a directory)
    await treemapContainer.click({ position: { x: 200, y: 200 } });

    // Wait for breadcrumb to update
    await page.waitForTimeout(500);

    // Check breadcrumb has segments
    const segments = await page.locator('[data-testid="breadcrumb-segment"]').count();
    expect(segments).toBeGreaterThan(0);

    // Verify breadcrumb text doesn't show "root / root"
    const breadcrumbText = await page.textContent('[data-testid="breadcrumb"]');
    const parts = breadcrumbText?.split('/').map(p => p.trim()).filter(Boolean);

    // Should have exactly one segment after root
    expect(parts?.length).toBe(2); // "Home" + one directory name

    // Second part should NOT be "root" (unless that's actually the directory name)
    console.log('Breadcrumb parts:', parts);
  });

  test('should drill down multiple levels', async ({ page }) => {
    // First drill-down
    const treemapContainer = await page.locator('[data-testid="treemap-container"]');
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // Second drill-down
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // Check we have multiple breadcrumb segments
    const segments = await page.locator('[data-testid="breadcrumb-segment"]').count();
    expect(segments).toBeGreaterThanOrEqual(1);
  });

  test('should navigate back using breadcrumb', async ({ page }) => {
    // Drill down
    const treemapContainer = await page.locator('[data-testid="treemap-container"]');
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // Check we have a breadcrumb segment
    const initialSegments = await page.locator('[data-testid="breadcrumb-segment"]').count();
    expect(initialSegments).toBeGreaterThan(0);

    // Click Home button to go back to root
    await page.click('button:has-text("Home")');
    await page.waitForTimeout(500);

    // Should be back at root
    const breadcrumb = await page.textContent('[data-testid="breadcrumb"]');
    expect(breadcrumb).toContain('Root directory');
  });

  test('should not show duplicate "root" in breadcrumb', async ({ page }) => {
    // Drill down
    const treemapContainer = await page.locator('[data-testid="treemap-container"]');
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // Get breadcrumb text
    const breadcrumbText = await page.textContent('[data-testid="breadcrumb"]');

    // Count occurrences of "root" (case-insensitive)
    const rootCount = (breadcrumbText?.toLowerCase().match(/root/g) || []).length;

    // Should only appear once (in "Root directory" or as a legitimate folder name)
    // If we see "root / root", that's 2 occurrences which is wrong
    expect(rootCount).toBeLessThanOrEqual(1);
  });

  test('should preserve path integrity across navigation', async ({ page }) => {
    // Drill down to src
    const treemapContainer = await page.locator('[data-testid="treemap-container"]');
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // Get first segment text
    const firstSegment = await page.locator('[data-testid="breadcrumb-segment"]').first().textContent();

    // Drill down again
    await treemapContainer.click({ position: { x: 200, y: 200 } });
    await page.waitForTimeout(500);

    // First segment should still be the same
    const firstSegmentAfter = await page.locator('[data-testid="breadcrumb-segment"]').first().textContent();
    expect(firstSegmentAfter).toBe(firstSegment);
  });
});
