/**
 * useAnalysisHandlers - Custom hook for AnalysisView event handlers
 *
 * Extracts event handling logic from the main AnalysisView component
 * to reduce its size and improve maintainability.
 */

import { useCallback } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import {
  useAnalysisActions,
  useSelectedFile,
  useDrillDownPath,
} from '@/store/analysisStore';
import type { TreeNode, AnalysisOptions } from '@/types/bindings';

interface UseAnalysisHandlersProps {
  repoPath: string;
  setRepoPath: (path: string) => void;
  setAnalysisOptions: (options: AnalysisOptions) => void;
  analyze: (path: string, options: AnalysisOptions) => Promise<void>;
  reset: () => void;
}

export function useAnalysisHandlers({
  repoPath,
  setRepoPath,
  setAnalysisOptions,
  analyze,
  reset,
}: UseAnalysisHandlersProps) {
  const { setSelectedFile, setDrillDownPath, toggleDeadCodeOverlay } = useAnalysisActions();
  const selectedFile = useSelectedFile();
  const drillDownPath = useDrillDownPath();

  const handleOptionsChange = useCallback(
    (newOptions: AnalysisOptions) => {
      setAnalysisOptions(newOptions);
      try {
        localStorage.setItem('analysisOptions', JSON.stringify(newOptions));
      } catch (e) {
        console.error('Failed to save analysis options:', e);
      }
    },
    [setAnalysisOptions]
  );

  const handleAnalyze = useCallback(async () => {
    const trimmedPath = repoPath.trim();
    if (!trimmedPath) {
      console.warn('[AnalysisView] Cannot analyze: empty path');
      return;
    }

    try {
      localStorage.setItem('lastRepoPath', trimmedPath);
    } catch (e) {
      console.warn('[AnalysisView] Failed to save repo path to localStorage:', e);
    }

    await analyze(trimmedPath, { enableDuplicates: false });
  }, [repoPath, analyze]);

  const handlePathKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === 'Enter' && repoPath.trim()) {
        handleAnalyze();
      }
    },
    [repoPath, handleAnalyze]
  );

  const handleBrowse = useCallback(async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select Repository Directory',
      });

      if (selected && typeof selected === 'string') {
        setRepoPath(selected);
      }
    } catch (error) {
      console.error('Failed to open directory picker:', error);
    }
  }, [setRepoPath]);

  const handleNodeClick = useCallback(
    (node: TreeNode) => {
      if (node.type === 'file') {
        setSelectedFile(node);
      } else {
        const newPath = [...drillDownPath, node.name];
        setDrillDownPath(newPath);
      }
    },
    [setSelectedFile, setDrillDownPath, drillDownPath]
  );

  const handleNodeHover = useCallback(
    () => {
      // Placeholder for future hover functionality
    },
    []
  );

  const handleBreadcrumbNavigate = useCallback(
    (index: number) => {
      if (index === -1) {
        setDrillDownPath([]);
      } else {
        setDrillDownPath(drillDownPath.slice(0, index + 1));
      }
    },
    [drillDownPath, setDrillDownPath]
  );

  const handleDetailPanelClose = useCallback(() => {
    setSelectedFile(null);
  }, [setSelectedFile]);

  const handleNavigateBack = useCallback(() => {
    if (selectedFile) {
      setSelectedFile(null);
    } else if (drillDownPath.length > 0) {
      setDrillDownPath(drillDownPath.slice(0, -1));
    }
  }, [selectedFile, drillDownPath, setSelectedFile, setDrillDownPath]);

  const handleReset = useCallback(() => {
    reset();
    setSelectedFile(null);
    setDrillDownPath([]);
  }, [reset, setSelectedFile, setDrillDownPath]);

  const handleDeadCodePanelClose = useCallback(() => {
    toggleDeadCodeOverlay();
  }, [toggleDeadCodeOverlay]);

  return {
    handleOptionsChange,
    handleAnalyze,
    handlePathKeyDown,
    handleBrowse,
    handleNodeClick,
    handleNodeHover,
    handleBreadcrumbNavigate,
    handleDetailPanelClose,
    handleNavigateBack,
    handleReset,
    handleDeadCodePanelClose,
  };
}
