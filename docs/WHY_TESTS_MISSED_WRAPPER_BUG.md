# Why Tests Didn't Catch the Wrapper Node Bug

## The Bug

**Issue:** Clicking on the treemap showed "Clicked node has no path" error.

**Root Cause:**
```typescript
// BEFORE (BROKEN):
data: echartsData?.children && echartsData.children.length > 0
  ? echartsData.children  // ← Passing array of children
  : [echartsData],
```

This created an ECharts wrapper container with:
- `children`: Array(13) with actual nodes
- `value`: 14743 (total LOC)
- `name`: undefined
- `path`: undefined ← **Missing!**

When clicking on this wrapper, `findNodeByPath()` failed because there was no path.

**Fix:**
```typescript
// AFTER (FIXED):
data: [echartsData],  // ← Pass root node with all properties
```

Now the root node has `name`, `path`, `type`, and `children`.

---

## Why Each Test Level Missed It

### 1. Unit Tests ❌ Didn't Catch It

**File:** `src/components/visualizations/Treemap.test.tsx`

**What they tested:**
- ✅ ECharts options structure
- ✅ Click handler receives correct TreeNode format
- ✅ findNodeByPath returns original node

**What they DIDN'T test:**
- ❌ The actual `data` array passed to ECharts series
- ❌ Whether wrapper nodes have required properties

**Why they missed it:**
```typescript
// Test expected old behavior:
expect(optionsArg.series[0].data).toHaveLength(2); // file1.ts and subdir
expect(optionsArg.series[0].data[0].name).toBe('file1.ts');
```

The test verified the data structure but didn't verify that EVERY node (including container nodes) has `name`, `path`, and `type`.

**How to catch it:**
```typescript
it('should ensure all nodes in data array have required properties', () => {
  render(<Treemap data={mockTreeData} />);

  const optionsArg = mockSetOption.mock.calls[0][0];
  const nodes = optionsArg.series[0].data;

  // CRITICAL: Every node must have path/name/type
  nodes.forEach((node, index) => {
    expect(node.name, `Node ${index} must have name`).toBeDefined();
    expect(node.path, `Node ${index} must have path`).toBeDefined();
    expect(node.type, `Node ${index} must have type`).toBeDefined();
  });
});
```

---

### 2. Integration Tests ❌ Didn't Catch It

**File:** `src/features/analysis/AnalysisView.drilldown.test.tsx`

**What they tested:**
- ✅ Drill-down path updates correctly
- ✅ Breadcrumb navigation works
- ✅ Selected file state updates

**What they DIDN'T test:**
- ❌ The actual Treemap component was **mocked**
- ❌ Real click handling with ECharts data

**Why they missed it:**
```typescript
// Treemap was mocked:
vi.mock('@/components/visualizations/Treemap', () => ({
  Treemap: vi.fn(({ onNodeClick, data }) => {
    return (
      <div>
        {data?.children?.map((child: TreeNode) => (
          <button onClick={() => onNodeClick(child)}>
            {child.name}
          </button>
        ))}
      </div>
    );
  }),
}));
```

The mock passed `child` (a TreeNode) directly to `onNodeClick`, bypassing the ECharts transformation and `findNodeByPath` logic entirely.

**How to catch it:**
Use the **real Treemap component** for at least one integration test:

```typescript
describe('Real Treemap integration', () => {
  // DON'T mock Treemap for this test
  it('should handle clicks with real Treemap and ECharts', () => {
    const onNodeClick = vi.fn();

    render(<Treemap data={mockTreeData} onNodeClick={onNodeClick} />);

    // Simulate ECharts click event with actual params structure
    const clickHandler = mockOn.mock.calls.find(c => c[0] === 'click')?.[1];

    // Click on the first node in the data array
    const firstNodeData = getEChartsData()[0]; // Helper to get actual ECharts data
    clickHandler({ data: firstNodeData });

    // Should call onNodeClick with original TreeNode
    expect(onNodeClick).toHaveBeenCalled();
  });
});
```

---

### 3. E2E Tests ❌ Didn't Catch It

**File:** `tests/e2e/treemap.spec.ts`

**What they tested:**
- ✅ UI workflows with mocked Tauri backend
- ✅ Drill-down navigation
- ✅ Keyboard navigation

**What they DIDN'T test:**
- ❌ Used **mocked Tauri data** that was already in correct format
- ❌ Never tested with real backend data structure

**Why they missed it:**
```typescript
await page.addInitScript(() => {
  (window as any).__TAURI_INTERNALS__ = {
    invoke: async (cmd: string) => {
      return {
        name: 'sample-repo',
        path: '/tests/fixtures/sample-repo',
        lastModified: '2025-12-18T00:00:00Z',  // ← Already correct format
        children: [...],  // ← Already correct structure
      };
    },
  };
});
```

The mock data was already in the expected TreeNode format, so it never exercised the actual Rust serialization or ECharts transformation.

**How to catch it:**
Add E2E tests that use the **real Tauri backend** (not mocked):

```typescript
describe('Real Tauri backend E2E', () => {
  it('should analyze repository and handle clicks', async ({ page }) => {
    // Start Tauri app (requires `npm run tauri dev` or built app)
    await page.goto('tauri://localhost');

    // Use real repository
    await page.fill('[data-testid="repository-path-input"]', './');
    await page.click('[data-testid="analyze-button"]');

    // Wait for real analysis to complete
    await page.waitForSelector('[data-testid="treemap-container"]');

    // Try clicking on actual rendered rectangles
    const treemap = await page.locator('[data-testid="treemap-node"]');
    await treemap.click({ position: { x: 100, y: 100 } });

    // Verify no errors in console
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') errors.push(msg.text());
    });

    expect(errors).toHaveLength(0);
  });
});
```

---

## Test Strategy Updates

### Layer 1: Unit Tests (Component Level)

**✅ IMPLEMENTED:** Validate ECharts data structure constraints

Added comprehensive validation test in `src/components/visualizations/Treemap.test.tsx`:

```typescript
/**
 * CRITICAL REGRESSION TEST: Prevent wrapper node bug
 *
 * This test validates that EVERY node in the ECharts data structure
 * (including the root and all nested children) has the required properties.
 */
it('should ensure all nodes in data array have required properties (path, name, type)', () => {
  render(<Treemap data={mockTreeData} />);

  const optionsArg = mockSetOption.mock.calls[0][0];
  const nodes = optionsArg.series[0].data;

  function validateNode(node: any, nodePath = 'root') {
    expect(node.name, `${nodePath}: must have name`).toBeDefined();
    expect(node.path, `${nodePath}: must have path`).toBeDefined();
    expect(node.type, `${nodePath}: must have type`).toBeDefined();

    expect(typeof node.name, `${nodePath}: name must be string`).toBe('string');
    expect(typeof node.path, `${nodePath}: path must be string`).toBe('string');
    expect(typeof node.type, `${nodePath}: type must be string`).toBe('string');
    expect(['file', 'directory'], `${nodePath}: type must be file or directory`).toContain(node.type);

    if (node.children && Array.isArray(node.children)) {
      node.children.forEach((child: any, i: number) => {
        validateNode(child, `${nodePath}.children[${i}]`);
      });
    }
  }

  nodes.forEach((node: any, i: number) => {
    validateNode(node, `data[${i}]`);
  });

  // Additional assertion: data array should have exactly 1 element (the root)
  expect(nodes).toHaveLength(1);
  expect(nodes[0].name).toBe('root');
});
```

**Status:** ✅ Test added and passing (61/61 tests pass)

### Layer 2: Integration Tests

**Add:** At least one test with real Treemap (not mocked)

```typescript
describe('Integration with real Treemap', () => {
  beforeEach(() => {
    // DON'T mock Treemap for these tests
    vi.unmock('@/components/visualizations/Treemap');
  });

  it('should handle drill-down with real Treemap component', async () => {
    render(<AnalysisView />);

    // Trigger analysis with real data
    await userEvent.type(input, '/test/path');
    await userEvent.click(analyzeButton);

    // Wait for render
    await waitFor(() => {
      expect(screen.getByTestId('treemap-container')).toBeInTheDocument();
    });

    // Try clicking on treemap
    const treemap = screen.getByTestId('treemap-node');
    await userEvent.click(treemap);

    // Should not have console errors
    expect(console.error).not.toHaveBeenCalled();
  });
});
```

### Layer 3: E2E Tests

**Add:** Tests with real Tauri backend

```typescript
describe('Real backend E2E', () => {
  it('should work with actual Tauri backend', async ({ page }) => {
    // Launch actual Tauri app
    await startTauriApp();

    // Analyze real repository
    await page.fill('[data-testid="repository-path-input"]', process.cwd());
    await page.click('[data-testid="analyze-button"]');

    // Wait for real analysis
    await page.waitForSelector('[data-testid="treemap-container"]', {
      timeout: 30000
    });

    // Monitor console errors
    const consoleErrors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    // Click on different areas of the treemap
    const canvas = page.locator('canvas[data-zr-dom-id]');
    await canvas.click({ position: { x: 100, y: 50 } });
    await canvas.click({ position: { x: 200, y: 50 } });
    await canvas.click({ position: { x: 300, y: 50 } });

    // Should have no errors
    expect(consoleErrors).toHaveLength(0);
  });
});
```

---

## Summary

| Test Level | What Missed It | How to Fix |
|------------|----------------|------------|
| **Unit Tests** | Didn't validate data structure constraints | Add validation for required properties on all nodes |
| **Integration Tests** | Mocked Treemap component | Add at least one test with real Treemap |
| **E2E Tests** | Mocked Tauri backend | Add tests with real backend |

**Key Principle:**
> "Test at least one path through the real system, not just mocked components."

**Updated Strategy:**
- 90% of tests can use mocks (fast, reliable)
- 10% of tests must use real components/backend (catch integration bugs)

This would have caught the wrapper node bug at multiple levels!

---

## Implementation Status

✅ **Bug Fixed:** Treemap.tsx now passes `[echartsData]` instead of `echartsData.children`
✅ **Unit Test Added:** Validation test ensures all nodes have required properties
✅ **Documentation Complete:** Comprehensive analysis of why each test level missed the bug
⏳ **Integration Tests:** Recommended improvements documented (not yet implemented)
⏳ **E2E Tests:** Recommended improvements documented (not yet implemented)

**All 61 Treemap unit tests passing**

**Commits:**
- `1914af3`: Added detailed click logging and localStorage persistence
- `954f55d`: Fixed Treemap data structure bug
- Current: Added regression test to prevent future occurrences
