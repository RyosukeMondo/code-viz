/**
 * Generic React hook for type-safe Tauri command invocation
 *
 * This hook wraps Tauri's invoke API with automatic loading/error state management,
 * cleanup on unmount, and TypeScript generics for type safety.
 *
 * @module hooks/useTauriCommand
 */

import { useState, useCallback, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * State returned by useTauriCommand hook
 */
interface TauriCommandState<T> {
  /** Result data (null until command completes successfully) */
  data: T | null;

  /** Loading state (true while command is executing) */
  loading: boolean;

  /** Error message (null if no error) */
  error: string | null;

  /** Current request ID (for log correlation) */
  requestId: string | null;

  /** Function to execute the Tauri command */
  execute: (...args: unknown[]) => Promise<void>;

  /** Function to reset state to initial values */
  reset: () => void;
}

/**
 * Options for useTauriCommand hook
 */
interface UseTauriCommandOptions {
  /** If true, execute the command immediately on mount */
  immediate?: boolean;

  /** Arguments to pass if immediate execution is enabled */
  immediateArgs?: unknown[];

  /** Callback fired when command succeeds */
  onSuccess?: (data: unknown) => void;

  /** Callback fired when command fails */
  onError?: (error: string) => void;
}

/**
 * Generic hook for invoking Tauri commands with automatic state management
 *
 * Provides type-safe command invocation with loading/error states and cleanup.
 * Uses TypeScript generics to infer command return type.
 *
 * @template T - The return type of the Tauri command
 * @param command - The name of the Tauri command to invoke
 * @param options - Optional configuration for the hook
 * @returns State object with data, loading, error, execute function, and reset function
 *
 * @example
 * ```typescript
 * // Basic usage
 * function MyComponent() {
 *   const { data, loading, error, execute } = useTauriCommand<TreeNode>('analyze_repository');
 *
 *   const handleAnalyze = () => {
 *     execute('/path/to/repo');
 *   };
 *
 *   if (loading) return <div>Loading...</div>;
 *   if (error) return <div>Error: {error}</div>;
 *   return <div>LOC: {data?.loc}</div>;
 * }
 * ```
 *
 * @example
 * ```typescript
 * // With immediate execution
 * const { data, loading } = useTauriCommand<TreeNode>('analyze_repository', {
 *   immediate: true,
 *   immediateArgs: ['/path/to/repo'],
 *   onSuccess: (tree) => console.log('Analysis complete!', tree),
 *   onError: (err) => console.error('Analysis failed:', err),
 * });
 * ```
 */
export function useTauriCommand<T = unknown>(
  command: string,
  options: UseTauriCommandOptions = {}
): TauriCommandState<T> {
  const { immediate = false, immediateArgs = [], onSuccess, onError } = options;

  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [requestId, setRequestId] = useState<string | null>(null);

  // Track if component is mounted to prevent state updates after unmount
  const isMountedRef = useRef(true);

  // Track ongoing requests to support cancellation
  const abortControllerRef = useRef<AbortController | null>(null);

  /**
   * Execute the Tauri command with the provided arguments
   */
  const execute = useCallback(
    async (...args: unknown[]) => {
      // Cancel any ongoing request
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }

      // Create new abort controller for this request
      abortControllerRef.current = new AbortController();

      if (!isMountedRef.current) return;

      // Generate unique request ID for correlation with backend logs
      const reqId = crypto.randomUUID();
      setRequestId(reqId);
      setLoading(true);
      setError(null);

      console.log(`[${reqId}] Starting Tauri command: ${command}`, args);

      try {
        // Convert args array to args object expected by Tauri
        // For single argument commands, pass the first arg directly
        // For multiple arguments, assume they're passed as an object
        let invokeArgs: Record<string, unknown> | undefined;

        if (args.length === 0) {
          // No arguments, just add request_id
          invokeArgs = { request_id: reqId };
        } else if (args.length === 1) {
          // Single argument - merge with request_id
          const firstArg = args[0];
          if (typeof firstArg === 'object' && firstArg !== null) {
            invokeArgs = { ...firstArg, request_id: reqId } as Record<
              string,
              unknown
            >;
          } else {
            // If it's a primitive, wrap it
            invokeArgs = { value: firstArg, request_id: reqId };
          }
        } else {
          // Multiple arguments - assume first is object
          invokeArgs = {
            ...(args[0] as Record<string, unknown>),
            request_id: reqId,
          };
        }

        const result = await invoke<T>(command, invokeArgs);

        // Check if request was aborted or component unmounted
        if (
          abortControllerRef.current.signal.aborted ||
          !isMountedRef.current
        ) {
          return;
        }

        console.log(`[${reqId}] Command completed successfully: ${command}`);
        setData(result);
        setLoading(false);

        if (onSuccess) {
          onSuccess(result);
        }
      } catch (err) {
        // Check if request was aborted or component unmounted
        if (
          abortControllerRef.current.signal.aborted ||
          !isMountedRef.current
        ) {
          return;
        }

        const errorMessage =
          err instanceof Error ? err.message : String(err);
        console.error(`[${reqId}] Command failed: ${command}`, errorMessage);
        setError(errorMessage);
        setLoading(false);

        if (onError) {
          onError(errorMessage);
        }
      }
    },
    [command, onSuccess, onError]
  );

  /**
   * Reset state to initial values
   */
  const reset = useCallback(() => {
    if (!isMountedRef.current) return;

    setData(null);
    setLoading(false);
    setError(null);
    setRequestId(null);
  }, []);

  // Execute immediately on mount if requested
  useEffect(() => {
    if (immediate) {
      execute(...immediateArgs);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [immediate]); // Only run once on mount

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      isMountedRef.current = false;
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    };
  }, []);

  return {
    data,
    loading,
    error,
    requestId,
    execute,
    reset,
  };
}
