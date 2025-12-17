/**
 * Breadcrumb component for drill-down navigation
 *
 * Displays the current drill-down path with clickable segments allowing users
 * to navigate back to parent directories. Includes a home button to return to root.
 */

import React from 'react';
import type { BreadcrumbProps } from '../../types';
import { formatPath } from '../../utils/formatting';

/**
 * Breadcrumb navigation component
 *
 * @param path - Array of path segments representing the drill-down path
 * @param onNavigate - Callback fired when a breadcrumb segment is clicked with the segment index
 */
export const Breadcrumb: React.FC<BreadcrumbProps> = ({ path, onNavigate }) => {
  /**
   * Handle keyboard navigation for breadcrumb segments
   * Supports Enter and Space keys for activation
   */
  const handleKeyDown = (event: React.KeyboardEvent, index: number) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onNavigate(index);
    }
  };

  /**
   * Handle home button navigation
   * Navigates to root (index -1 represents root)
   */
  const handleHomeClick = () => {
    onNavigate(-1);
  };

  /**
   * Handle home button keyboard navigation
   */
  const handleHomeKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleHomeClick();
    }
  };

  return (
    <nav
      data-testid="breadcrumb"
      aria-label="Breadcrumb navigation"
      className="flex items-center space-x-2 text-sm text-gray-700 dark:text-gray-300"
    >
      {/* Home button */}
      <button
        type="button"
        onClick={handleHomeClick}
        onKeyDown={handleHomeKeyDown}
        aria-label="Navigate to root"
        className="inline-flex items-center px-3 py-1.5 rounded-md font-medium transition-colors
                   hover:bg-gray-100 dark:hover:bg-gray-700
                   focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                   dark:focus:ring-offset-gray-900"
      >
        <svg
          className="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
          />
        </svg>
        <span className="ml-1.5">Home</span>
      </button>

      {/* Path segments */}
      {path.length > 0 && (
        <>
          {path.map((segment, index) => (
            <React.Fragment key={`${segment}-${index}`}>
              {/* Separator */}
              <span className="text-gray-400 dark:text-gray-600" aria-hidden="true">
                /
              </span>

              {/* Breadcrumb segment */}
              <button
                type="button"
                onClick={() => onNavigate(index)}
                onKeyDown={(e) => handleKeyDown(e, index)}
                data-testid="breadcrumb-segment"
                aria-label={`Navigate to ${segment}`}
                aria-current={index === path.length - 1 ? 'page' : undefined}
                className={`inline-flex items-center px-3 py-1.5 rounded-md font-medium transition-colors
                           focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
                           dark:focus:ring-offset-gray-900
                           ${
                             index === path.length - 1
                               ? 'text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/30'
                               : 'hover:bg-gray-100 dark:hover:bg-gray-700'
                           }`}
              >
                {formatPath(segment, { maxLength: 30, position: 'end' })}
              </button>
            </React.Fragment>
          ))}
        </>
      )}

      {/* Empty state when at root */}
      {path.length === 0 && (
        <span className="text-gray-500 dark:text-gray-500 italic ml-2">
          Root directory
        </span>
      )}
    </nav>
  );
};

export default Breadcrumb;
