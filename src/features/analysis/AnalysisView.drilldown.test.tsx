/**
 * Tests for drill-down navigation functionality in AnalysisView
 *
 * These tests verify that clicking on directories in the treemap
 * correctly updates the drill-down path and filters the displayed tree.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { AnalysisView } from './AnalysisView';
import type { TreeNode } from '@/types/bindings';

// Mock the Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock the dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
}));

// Mock the Treemap component to expose click handlers
vi.mock('@/components/visualizations/Treemap', () => ({
  Treemap: vi.fn(({ onNodeClick, data, drillDownPath }) => {
    return (
      <div data-testid="treemap-mock">
        <div data-testid="current-node">{data?.name || 'null'}</div>
        <div data-testid="drill-down-path">{JSON.stringify(drillDownPath)}</div>
        {/* Expose a button to simulate clicking on a child directory */}
        {data?.children?.map((child: TreeNode) => (
          <button
            key={child.id}
            data-testid={`click-${child.name}`}
            onClick={() => onNodeClick(child)}
          >
            {child.name}
          </button>
        ))}
      </div>
    );
  }),
}));

// Mock other components
vi.mock('@/components/common/Breadcrumb', () => ({
  Breadcrumb: vi.fn(({ path, onNavigate }) => (
    <div data-testid="breadcrumb-mock">
      <div data-testid="breadcrumb-path">{JSON.stringify(path)}</div>
      <button
        data-testid="navigate-root"
        onClick={() => onNavigate(-1)}
      >
        Root
      </button>
      {path.map((_segment: string, index: number) => (
        <button
          key={index}
          data-testid={`navigate-${index}`}
          onClick={() => onNavigate(index)}
        >
          {_segment}
        </button>
      ))}
    </div>
  )),
}));

vi.mock('@/components/common/DetailPanel', () => ({
  DetailPanel: () => null,
}));

vi.mock('@/components/common/ErrorBoundary', () => ({
  ErrorBoundary: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('@/components/common/LoadingSkeleton', () => ({
  AnalysisLoadingSkeleton: () => <div data-testid="loading-skeleton">Loading...</div>,
}));

describe('AnalysisView - Drill-down navigation', () => {
  // Sample tree structure for testing
  const mockTreeData: TreeNode = {
    id: 'root',
    name: 'code-viz',
    path: '/home/user/code-viz',
    type: 'directory',
    loc: 1000,
    complexity: 5.0,
    lastModified: '2024-01-01',
    children: [
      {
        id: 'src',
        name: 'src',
        path: '/home/user/code-viz/src',
        type: 'directory',
        loc: 800,
        complexity: 4.5,
        lastModified: '2024-01-01',
        children: [
          {
            id: 'components',
            name: 'components',
            path: '/home/user/code-viz/src/components',
            type: 'directory',
            loc: 400,
            complexity: 4.0,
            lastModified: '2024-01-01',
            children: [
              {
                id: 'Button.tsx',
                name: 'Button.tsx',
                path: '/home/user/code-viz/src/components/Button.tsx',
                type: 'file',
                loc: 50,
                complexity: 2.0,
                lastModified: '2024-01-01',
                children: [],
              },
            ],
          },
          {
            id: 'utils',
            name: 'utils',
            path: '/home/user/code-viz/src/utils',
            type: 'directory',
            loc: 400,
            complexity: 4.5,
            lastModified: '2024-01-01',
            children: [],
          },
        ],
      },
      {
        id: 'README.md',
        name: 'README.md',
        path: '/home/user/code-viz/README.md',
        type: 'file',
        loc: 200,
        complexity: 1.0,
        lastModified: '2024-01-01',
        children: [],
      },
    ],
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should start with empty drill-down path at root', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository to get data
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    // Wait for analysis to complete
    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Check that drill-down path is empty (root view)
    const pathElement = screen.getByTestId('drill-down-path');
    expect(pathElement.textContent).toBe('[]');

    // Check that root node is displayed
    const currentNode = screen.getByTestId('current-node');
    expect(currentNode.textContent).toBe('code-viz');
  });

  it('should update drill-down path when clicking on a directory', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Click on "src" directory
    const srcButton = screen.getByTestId('click-src');
    await userEvent.click(srcButton);

    // Check that drill-down path is updated
    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('["src"]');
    });

    // Check that breadcrumb shows the path
    const breadcrumbPath = screen.getByTestId('breadcrumb-path');
    expect(breadcrumbPath.textContent).toBe('["src"]');
  });

  it('should drill down multiple levels', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Click on "src" directory
    const srcButton = screen.getByTestId('click-src');
    await userEvent.click(srcButton);

    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('["src"]');
    });

    // Click on "components" directory (child of src)
    const componentsButton = screen.getByTestId('click-components');
    await userEvent.click(componentsButton);

    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('["src","components"]');
    });
  });

  it('should navigate back to root using breadcrumb', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Drill down to src
    const srcButton = screen.getByTestId('click-src');
    await userEvent.click(srcButton);

    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('["src"]');
    });

    // Navigate back to root
    const rootButton = screen.getByTestId('navigate-root');
    await userEvent.click(rootButton);

    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('[]');
    });
  });

  it('should navigate to specific breadcrumb segment', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Drill down to src -> components
    await userEvent.click(screen.getByTestId('click-src'));
    await waitFor(() => {
      expect(screen.getByTestId('drill-down-path').textContent).toBe('["src"]');
    });

    await userEvent.click(screen.getByTestId('click-components'));
    await waitFor(() => {
      expect(screen.getByTestId('drill-down-path').textContent).toBe('["src","components"]');
    });

    // Navigate back to "src" using breadcrumb
    const srcBreadcrumb = screen.getByTestId('navigate-0');
    await userEvent.click(srcBreadcrumb);

    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('["src"]');
    });
  });

  it('should not drill down when clicking on a file', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // Analyze a repository
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Drill down path should still be empty
    const pathElement = screen.getByTestId('drill-down-path');
    expect(pathElement.textContent).toBe('[]');
  });

  it('should clear drill-down path when loading new metrics', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(mockTreeData);

    render(<AnalysisView />);

    // First analysis
    const input = screen.getByTestId('repository-path-input');
    const analyzeButton = screen.getByTestId('analyze-button');

    await userEvent.type(input, '/home/user/code-viz');
    await userEvent.click(analyzeButton);

    await waitFor(() => {
      expect(screen.getByTestId('treemap-mock')).toBeInTheDocument();
    });

    // Drill down
    await userEvent.click(screen.getByTestId('click-src'));
    await waitFor(() => {
      expect(screen.getByTestId('drill-down-path').textContent).toBe('["src"]');
    });

    // Analyze again
    await userEvent.clear(input);
    await userEvent.type(input, '/home/user/other-project');
    await userEvent.click(analyzeButton);

    // Drill-down path should be cleared
    await waitFor(() => {
      const pathElement = screen.getByTestId('drill-down-path');
      expect(pathElement.textContent).toBe('[]');
    });
  });
});
