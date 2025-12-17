/**
 * Formatting utilities for consistent data display across UI
 * Handles numbers, paths, bytes, and dates with proper edge case handling
 */

/**
 * Formats a number with thousands separators
 * Used primarily for displaying LOC (Lines of Code)
 *
 * @param value - Number to format
 * @returns Formatted string with commas (e.g., "1,234,567")
 */
export function formatNumber(value: number | null | undefined): string {
  // Handle edge cases
  if (value === null || value === undefined) {
    return '0';
  }

  if (typeof value !== 'number' || isNaN(value)) {
    return '0';
  }

  // Use Intl.NumberFormat for locale-aware formatting
  return new Intl.NumberFormat('en-US').format(Math.round(value));
}

/**
 * Configuration options for path truncation
 */
export interface PathTruncateOptions {
  /** Maximum length before truncation (default: 50) */
  maxLength?: number;
  /** Position to truncate: 'start' | 'middle' | 'end' (default: 'middle') */
  position?: 'start' | 'middle' | 'end';
  /** Ellipsis character (default: '...') */
  ellipsis?: string;
}

/**
 * Truncates a file path for display
 * Preserves important parts (filename and extension) while shortening directory path
 *
 * @param path - File path to format
 * @param options - Truncation configuration
 * @returns Truncated path string
 */
export function formatPath(
  path: string | null | undefined,
  options: PathTruncateOptions = {}
): string {
  // Handle edge cases
  if (!path || typeof path !== 'string') {
    return '';
  }

  const {
    maxLength = 50,
    position = 'middle',
    ellipsis = '...',
  } = options;

  // No truncation needed
  if (path.length <= maxLength) {
    return path;
  }

  // Truncate based on position
  switch (position) {
    case 'start': {
      // Keep the end (filename)
      const keepLength = maxLength - ellipsis.length;
      return ellipsis + path.slice(-keepLength);
    }

    case 'end': {
      // Keep the start (directory structure)
      const keepLength = maxLength - ellipsis.length;
      return path.slice(0, keepLength) + ellipsis;
    }

    case 'middle':
    default: {
      // Preserve directory start and filename
      // Split by path separator to identify filename
      const parts = path.split(/[/\\]/);
      const filename = parts[parts.length - 1];

      // If filename itself is too long, truncate in middle
      if (filename.length >= maxLength - ellipsis.length) {
        const halfLength = Math.floor((maxLength - ellipsis.length) / 2);
        return filename.slice(0, halfLength) + ellipsis + filename.slice(-halfLength);
      }

      // Calculate space for directory path
      const availableForDir = maxLength - filename.length - ellipsis.length - 1; // -1 for separator

      if (availableForDir <= 0) {
        // Just show filename
        return ellipsis + '/' + filename;
      }

      // Get directory path (everything except filename)
      const dirPath = parts.slice(0, -1).join('/');

      // Truncate directory from start
      const truncatedDir = dirPath.length > availableForDir
        ? ellipsis + dirPath.slice(-(availableForDir - ellipsis.length))
        : dirPath;

      return truncatedDir + '/' + filename;
    }
  }
}

/**
 * Formats bytes into human-readable sizes
 *
 * @param bytes - Number of bytes
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted string (e.g., "1.23 MB")
 */
export function formatBytes(
  bytes: number | null | undefined,
  decimals: number = 2
): string {
  // Handle edge cases
  if (bytes === null || bytes === undefined) {
    return '0 B';
  }

  if (typeof bytes !== 'number' || isNaN(bytes) || bytes < 0) {
    return '0 B';
  }

  if (bytes === 0) {
    return '0 B';
  }

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = bytes / Math.pow(k, i);

  return `${size.toFixed(dm)} ${sizes[i]}`;
}

/**
 * Formats a date into a human-readable string
 * Shows relative time for recent dates, absolute for older dates
 *
 * @param date - Date to format (Date object, ISO string, or timestamp)
 * @param options - Intl.DateTimeFormatOptions for customization
 * @returns Formatted date string
 */
export function formatDate(
  date: Date | string | number | null | undefined,
  options?: Intl.DateTimeFormatOptions
): string {
  // Handle edge cases
  if (!date) {
    return 'Unknown';
  }

  let dateObj: Date;

  try {
    // Convert to Date object
    if (date instanceof Date) {
      dateObj = date;
    } else if (typeof date === 'string') {
      dateObj = new Date(date);
    } else if (typeof date === 'number') {
      dateObj = new Date(date);
    } else {
      return 'Unknown';
    }

    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      return 'Unknown';
    }

    // Use Intl.DateTimeFormat for locale-aware formatting
    const defaultOptions: Intl.DateTimeFormatOptions = {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    };

    const formatOptions = options || defaultOptions;

    return new Intl.DateTimeFormat('en-US', formatOptions).format(dateObj);
  } catch (error) {
    return 'Unknown';
  }
}

/**
 * Formats a date into a relative time string (e.g., "2 hours ago")
 * Falls back to absolute date if too old
 *
 * @param date - Date to format
 * @returns Relative time string
 */
export function formatRelativeDate(
  date: Date | string | number | null | undefined
): string {
  // Handle edge cases
  if (!date) {
    return 'Unknown';
  }

  let dateObj: Date;

  try {
    // Convert to Date object
    if (date instanceof Date) {
      dateObj = date;
    } else if (typeof date === 'string') {
      dateObj = new Date(date);
    } else if (typeof date === 'number') {
      dateObj = new Date(date);
    } else {
      return 'Unknown';
    }

    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      return 'Unknown';
    }

    const now = new Date();
    const diffMs = now.getTime() - dateObj.getTime();
    const diffSec = Math.floor(diffMs / 1000);
    const diffMin = Math.floor(diffSec / 60);
    const diffHour = Math.floor(diffMin / 60);
    const diffDay = Math.floor(diffHour / 24);

    // Less than 1 minute
    if (diffSec < 60) {
      return 'just now';
    }

    // Less than 1 hour
    if (diffMin < 60) {
      return `${diffMin} ${diffMin === 1 ? 'minute' : 'minutes'} ago`;
    }

    // Less than 24 hours
    if (diffHour < 24) {
      return `${diffHour} ${diffHour === 1 ? 'hour' : 'hours'} ago`;
    }

    // Less than 7 days
    if (diffDay < 7) {
      return `${diffDay} ${diffDay === 1 ? 'day' : 'days'} ago`;
    }

    // More than 7 days - use absolute date
    return formatDate(dateObj, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  } catch (error) {
    return 'Unknown';
  }
}
