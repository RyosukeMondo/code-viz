/**
 * API Client - Auto-detects Tauri vs Web mode
 *
 * This module provides a unified API client that automatically detects
 * whether it's running in Tauri (desktop) or Web mode and uses the
 * appropriate transport (IPC vs HTTP).
 *
 * SSOT: Both modes call the same backend handlers (code-viz-api),
 * only the transport layer differs.
 */

import { invoke } from '@tauri-apps/api/core';
import type { TreeNode } from '../types';
import type { DeadCodeResult } from '../types/bindings';

/**
 * Detect if running in Tauri desktop app
 */
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

console.log(`[API Client] Running in ${isTauri ? 'Tauri (Desktop)' : 'Web'} mode`);

/**
 * Analyze a repository
 *
 * @param path - Absolute path to the repository
 * @param requestId - Optional request ID for correlation
 * @returns Tree structure with metrics
 */
export async function analyzeRepository(
  path: string,
  requestId?: string
): Promise<TreeNode> {
  if (isTauri) {
    // Tauri mode: Use IPC
    console.log('[API Client] Using Tauri IPC for analyze_repository');
    return await invoke<TreeNode>('analyze_repository', {
      path,
      requestId,
    });
  } else {
    // Web mode: Use HTTP REST API
    console.log('[API Client] Using HTTP REST API for /api/analyze');
    const response = await fetch('/api/analyze', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        path,
        requestId,
      }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || 'Analysis failed');
    }

    return await response.json();
  }
}

/**
 * Analyze dead code in a repository
 *
 * @param path - Absolute path to the repository
 * @param minConfidence - Minimum confidence score (0-100)
 * @param requestId - Optional request ID for correlation
 * @returns Dead code analysis results
 */
export async function analyzeDeadCode(
  path: string,
  minConfidence: number = 70,
  requestId?: string
): Promise<DeadCodeResult> {
  if (isTauri) {
    // Tauri mode: Use IPC
    console.log('[API Client] Using Tauri IPC for analyze_dead_code_command');
    return await invoke<DeadCodeResult>('analyze_dead_code_command', {
      path,
      minConfidence,
      requestId,
    });
  } else {
    // Web mode: Use HTTP REST API
    console.log('[API Client] Using HTTP REST API for /api/dead-code');
    const response = await fetch('/api/dead-code', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        path,
        minConfidence,
        requestId,
      }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || 'Dead code analysis failed');
    }

    return await response.json();
  }
}

/**
 * Health check (Web mode only)
 *
 * @returns Health status
 */
export async function healthCheck(): Promise<{ status: string; service: string }> {
  if (isTauri) {
    // Tauri doesn't need health check
    return { status: 'healthy', service: 'code-viz-tauri' };
  } else {
    const response = await fetch('/api/health');
    return await response.json();
  }
}

/**
 * Get current mode
 */
export function getMode(): 'tauri' | 'web' {
  return isTauri ? 'tauri' : 'web';
}
