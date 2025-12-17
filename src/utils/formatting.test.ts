import { describe, it, expect } from 'vitest'
import {
  formatNumber,
  formatPath,
  formatBytes,
  formatDate,
  formatRelativeDate,
  type PathTruncateOptions,
} from './formatting'

describe('formatting utilities', () => {
  describe('formatNumber', () => {
    describe('valid numbers', () => {
      it('should format small numbers without separators', () => {
        expect(formatNumber(0)).toBe('0')
        expect(formatNumber(1)).toBe('1')
        expect(formatNumber(99)).toBe('99')
        expect(formatNumber(999)).toBe('999')
      })

      it('should format numbers with thousands separators', () => {
        expect(formatNumber(1000)).toBe('1,000')
        expect(formatNumber(1234)).toBe('1,234')
        expect(formatNumber(12345)).toBe('12,345')
        expect(formatNumber(123456)).toBe('123,456')
        expect(formatNumber(1234567)).toBe('1,234,567')
      })

      it('should round decimal numbers', () => {
        expect(formatNumber(1234.4)).toBe('1,234')
        expect(formatNumber(1234.5)).toBe('1,235')
        expect(formatNumber(1234.9)).toBe('1,235')
      })

      it('should handle negative numbers', () => {
        expect(formatNumber(-1234)).toBe('-1,234')
        expect(formatNumber(-1234567)).toBe('-1,234,567')
      })
    })

    describe('edge cases', () => {
      it('should return "0" for null', () => {
        expect(formatNumber(null)).toBe('0')
      })

      it('should return "0" for undefined', () => {
        expect(formatNumber(undefined)).toBe('0')
      })

      it('should return "0" for NaN', () => {
        expect(formatNumber(NaN)).toBe('0')
      })

      it('should return "0" for non-number values', () => {
        expect(formatNumber('invalid' as any)).toBe('0')
      })
    })
  })

  describe('formatPath', () => {
    describe('short paths (no truncation needed)', () => {
      it('should return path unchanged if shorter than maxLength', () => {
        expect(formatPath('src/file.ts')).toBe('src/file.ts')
        expect(formatPath('a/b/c.js')).toBe('a/b/c.js')
      })

      it('should return path unchanged if exactly maxLength', () => {
        const path = 'a'.repeat(50)
        expect(formatPath(path, { maxLength: 50 })).toBe(path)
      })
    })

    describe('truncation - middle position (default)', () => {
      it('should preserve filename and truncate directory path', () => {
        const longPath = 'very/long/directory/path/with/many/segments/file.ts'
        const result = formatPath(longPath, { maxLength: 30 })

        expect(result).toContain('file.ts')
        expect(result).toContain('...')
        expect(result.length).toBeLessThanOrEqual(30)
      })

      it('should handle very long filenames by truncating in middle', () => {
        const longFilename = 'a'.repeat(60) + '.ts'
        const result = formatPath(longFilename, { maxLength: 30 })

        expect(result).toContain('...')
        expect(result.length).toBeLessThanOrEqual(30)
        expect(result.startsWith('a')).toBe(true)
        expect(result.endsWith('.ts')).toBe(true)
      })

      it('should handle paths with backslashes (Windows)', () => {
        const windowsPath = 'C:\\Users\\Documents\\Projects\\file.ts'
        const result = formatPath(windowsPath, { maxLength: 30 })

        expect(result).toContain('file.ts')
        expect(result.length).toBeLessThanOrEqual(30)
      })
    })

    describe('truncation - start position', () => {
      it('should keep the end (filename) and truncate from start', () => {
        const longPath = 'very/long/directory/path/with/many/segments/file.ts'
        const result = formatPath(longPath, { maxLength: 30, position: 'start' })

        expect(result.startsWith('...')).toBe(true)
        expect(result.endsWith('file.ts')).toBe(true)
        expect(result.length).toBeLessThanOrEqual(30)
      })
    })

    describe('truncation - end position', () => {
      it('should keep the start and truncate from end', () => {
        const longPath = 'very/long/directory/path/with/many/segments/file.ts'
        const result = formatPath(longPath, { maxLength: 30, position: 'end' })

        expect(result.startsWith('very/')).toBe(true)
        expect(result.endsWith('...')).toBe(true)
        expect(result.length).toBeLessThanOrEqual(30)
      })
    })

    describe('custom ellipsis', () => {
      it('should use custom ellipsis character', () => {
        const longPath = 'very/long/directory/path/with/many/segments/file.ts'
        const result = formatPath(longPath, {
          maxLength: 30,
          position: 'end',
          ellipsis: '…',
        })

        expect(result).toContain('…')
        expect(result).not.toContain('...')
      })
    })

    describe('edge cases', () => {
      it('should return empty string for null', () => {
        expect(formatPath(null)).toBe('')
      })

      it('should return empty string for undefined', () => {
        expect(formatPath(undefined)).toBe('')
      })

      it('should return empty string for non-string values', () => {
        expect(formatPath(123 as any)).toBe('')
      })

      it('should return empty string for empty string', () => {
        expect(formatPath('')).toBe('')
      })

      it('should handle single-character paths', () => {
        expect(formatPath('a')).toBe('a')
      })
    })
  })

  describe('formatBytes', () => {
    describe('valid byte sizes', () => {
      it('should format bytes (< 1KB)', () => {
        expect(formatBytes(0)).toBe('0 B')
        expect(formatBytes(1)).toBe('1.00 B')
        expect(formatBytes(512)).toBe('512.00 B')
        expect(formatBytes(1023)).toBe('1023.00 B')
      })

      it('should format kilobytes', () => {
        expect(formatBytes(1024)).toBe('1.00 KB')
        expect(formatBytes(1536)).toBe('1.50 KB')
        expect(formatBytes(102400)).toBe('100.00 KB')
      })

      it('should format megabytes', () => {
        expect(formatBytes(1048576)).toBe('1.00 MB')
        expect(formatBytes(5242880)).toBe('5.00 MB')
      })

      it('should format gigabytes', () => {
        expect(formatBytes(1073741824)).toBe('1.00 GB')
        expect(formatBytes(5368709120)).toBe('5.00 GB')
      })

      it('should format terabytes', () => {
        expect(formatBytes(1099511627776)).toBe('1.00 TB')
      })

      it('should format petabytes', () => {
        expect(formatBytes(1125899906842624)).toBe('1.00 PB')
      })
    })

    describe('custom decimal places', () => {
      it('should format with 0 decimals', () => {
        expect(formatBytes(1536, 0)).toBe('2 KB')
        expect(formatBytes(1024000, 0)).toBe('1000 KB')
      })

      it('should format with 1 decimal', () => {
        expect(formatBytes(1536, 1)).toBe('1.5 KB')
        expect(formatBytes(5242880, 1)).toBe('5.0 MB')
      })

      it('should format with 3 decimals', () => {
        expect(formatBytes(1536, 3)).toBe('1.500 KB')
      })

      it('should handle negative decimals parameter', () => {
        expect(formatBytes(1536, -1)).toBe('2 KB')
      })
    })

    describe('edge cases', () => {
      it('should return "0 B" for null', () => {
        expect(formatBytes(null)).toBe('0 B')
      })

      it('should return "0 B" for undefined', () => {
        expect(formatBytes(undefined)).toBe('0 B')
      })

      it('should return "0 B" for NaN', () => {
        expect(formatBytes(NaN)).toBe('0 B')
      })

      it('should return "0 B" for negative numbers', () => {
        expect(formatBytes(-1024)).toBe('0 B')
        expect(formatBytes(-100)).toBe('0 B')
      })

      it('should return "0 B" for non-number values', () => {
        expect(formatBytes('invalid' as any)).toBe('0 B')
      })
    })
  })

  describe('formatDate', () => {
    describe('valid dates', () => {
      it('should format Date object', () => {
        const date = new Date('2024-01-15T10:30:00Z')
        const result = formatDate(date)

        expect(result).toBeTruthy()
        expect(result).not.toBe('Unknown')
        expect(result).toContain('Jan')
        expect(result).toContain('15')
        expect(result).toContain('2024')
      })

      it('should format ISO string', () => {
        const result = formatDate('2024-01-15T10:30:00Z')

        expect(result).toBeTruthy()
        expect(result).not.toBe('Unknown')
        expect(result).toContain('Jan')
        expect(result).toContain('15')
        expect(result).toContain('2024')
      })

      it('should format timestamp (number)', () => {
        const timestamp = new Date('2024-01-15T10:30:00Z').getTime()
        const result = formatDate(timestamp)

        expect(result).toBeTruthy()
        expect(result).not.toBe('Unknown')
      })
    })

    describe('custom format options', () => {
      it('should respect custom format options', () => {
        const date = new Date('2024-01-15T10:30:00Z')
        const result = formatDate(date, {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
        })

        expect(result).toContain('January')
        expect(result).toContain('15')
        expect(result).toContain('2024')
      })

      it('should format with time when specified', () => {
        const date = new Date('2024-01-15T10:30:00Z')
        const result = formatDate(date, {
          hour: '2-digit',
          minute: '2-digit',
        })

        expect(result).toMatch(/\d{1,2}:\d{2}/)
      })
    })

    describe('edge cases', () => {
      it('should return "Unknown" for null', () => {
        expect(formatDate(null)).toBe('Unknown')
      })

      it('should return "Unknown" for undefined', () => {
        expect(formatDate(undefined)).toBe('Unknown')
      })

      it('should return "Unknown" for invalid date string', () => {
        expect(formatDate('invalid-date')).toBe('Unknown')
      })

      it('should return "Unknown" for invalid Date object', () => {
        expect(formatDate(new Date('invalid'))).toBe('Unknown')
      })

      it('should return "Unknown" for non-date values', () => {
        expect(formatDate({} as any)).toBe('Unknown')
      })
    })
  })

  describe('formatRelativeDate', () => {
    describe('recent dates', () => {
      it('should return "just now" for dates < 1 minute ago', () => {
        const now = new Date()
        const recent = new Date(now.getTime() - 30 * 1000) // 30 seconds ago

        expect(formatRelativeDate(recent)).toBe('just now')
      })

      it('should return minutes for dates < 1 hour ago', () => {
        const now = new Date()
        const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000)

        const result = formatRelativeDate(fiveMinutesAgo)
        expect(result).toContain('minute')
        expect(result).toContain('ago')
      })

      it('should use singular "minute" for 1 minute', () => {
        const now = new Date()
        const oneMinuteAgo = new Date(now.getTime() - 1 * 60 * 1000)

        const result = formatRelativeDate(oneMinuteAgo)
        expect(result).toBe('1 minute ago')
      })

      it('should use plural "minutes" for multiple minutes', () => {
        const now = new Date()
        const twoMinutesAgo = new Date(now.getTime() - 2 * 60 * 1000)

        const result = formatRelativeDate(twoMinutesAgo)
        expect(result).toBe('2 minutes ago')
      })

      it('should return hours for dates < 24 hours ago', () => {
        const now = new Date()
        const threeHoursAgo = new Date(now.getTime() - 3 * 60 * 60 * 1000)

        const result = formatRelativeDate(threeHoursAgo)
        expect(result).toContain('hour')
        expect(result).toContain('ago')
      })

      it('should use singular "hour" for 1 hour', () => {
        const now = new Date()
        const oneHourAgo = new Date(now.getTime() - 1 * 60 * 60 * 1000)

        const result = formatRelativeDate(oneHourAgo)
        expect(result).toBe('1 hour ago')
      })

      it('should return days for dates < 7 days ago', () => {
        const now = new Date()
        const twoDaysAgo = new Date(now.getTime() - 2 * 24 * 60 * 60 * 1000)

        const result = formatRelativeDate(twoDaysAgo)
        expect(result).toContain('day')
        expect(result).toContain('ago')
      })

      it('should use singular "day" for 1 day', () => {
        const now = new Date()
        const oneDayAgo = new Date(now.getTime() - 1 * 24 * 60 * 60 * 1000)

        const result = formatRelativeDate(oneDayAgo)
        expect(result).toBe('1 day ago')
      })
    })

    describe('old dates', () => {
      it('should return absolute date for dates > 7 days ago', () => {
        const oldDate = new Date('2024-01-01T00:00:00Z')
        const result = formatRelativeDate(oldDate)

        // Should not contain "ago"
        expect(result).not.toContain('ago')
        // Should contain date components
        expect(result).toContain('Jan')
        expect(result).toContain('1')
        expect(result).toContain('2024')
      })
    })

    describe('edge cases', () => {
      it('should return "Unknown" for null', () => {
        expect(formatRelativeDate(null)).toBe('Unknown')
      })

      it('should return "Unknown" for undefined', () => {
        expect(formatRelativeDate(undefined)).toBe('Unknown')
      })

      it('should return "Unknown" for invalid date string', () => {
        expect(formatRelativeDate('invalid-date')).toBe('Unknown')
      })

      it('should return "Unknown" for invalid Date object', () => {
        expect(formatRelativeDate(new Date('invalid'))).toBe('Unknown')
      })

      it('should handle ISO string dates', () => {
        const now = new Date()
        const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000).toISOString()

        const result = formatRelativeDate(fiveMinutesAgo)
        expect(result).toContain('minute')
        expect(result).toContain('ago')
      })

      it('should handle timestamp dates', () => {
        const now = new Date()
        const fiveMinutesAgo = now.getTime() - 5 * 60 * 1000

        const result = formatRelativeDate(fiveMinutesAgo)
        expect(result).toContain('minute')
        expect(result).toContain('ago')
      })
    })
  })
})
