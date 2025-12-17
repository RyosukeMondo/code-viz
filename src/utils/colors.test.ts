import { describe, it, expect } from 'vitest'
import { complexityToColor, getComplexityGradient, getComplexityLabel } from './colors'

describe('colors utilities', () => {
  describe('complexityToColor', () => {
    describe('valid complexity scores', () => {
      it('should return green for score 0 (low complexity)', () => {
        const color = complexityToColor(0)
        expect(color).toBe('#22c55e')
      })

      it('should return green-yellow blend for score 15', () => {
        const color = complexityToColor(15)
        // Should be between green and yellow
        expect(color).toMatch(/^#[0-9a-f]{6}$/i)
        expect(color).not.toBe('#22c55e')
        expect(color).not.toBe('#eab308')
      })

      it('should return yellow for score 30 (medium complexity)', () => {
        const color = complexityToColor(30)
        expect(color).toBe('#eab308')
      })

      it('should return yellow-red blend for score 45', () => {
        const color = complexityToColor(45)
        // Should be between yellow and red
        expect(color).toMatch(/^#[0-9a-f]{6}$/i)
        expect(color).not.toBe('#eab308')
        expect(color).not.toBe('#ef4444')
      })

      it('should return red for score 60 (high complexity)', () => {
        const color = complexityToColor(60)
        expect(color).toBe('#ef4444')
      })

      it('should return darker red for score 80', () => {
        const color = complexityToColor(80)
        // Should be darker than base red
        expect(color).toMatch(/^#[0-9a-f]{6}$/i)
        expect(color).not.toBe('#ef4444')
      })

      it('should return darkest red for score 100 (very high complexity)', () => {
        const color = complexityToColor(100)
        expect(color).toBe('#dc2626')
      })
    })

    describe('edge cases', () => {
      it('should clamp negative values to 0 (green)', () => {
        const color = complexityToColor(-10)
        expect(color).toBe('#22c55e')
      })

      it('should clamp values above 100 to 100 (darkest red)', () => {
        const color = complexityToColor(150)
        expect(color).toBe('#dc2626')
      })

      it('should return gray for NaN', () => {
        const color = complexityToColor(NaN)
        expect(color).toBe('#94a3b8')
      })

      it('should return gray for non-number values', () => {
        const color = complexityToColor('invalid' as any)
        expect(color).toBe('#94a3b8')
      })

      it('should return gray for undefined', () => {
        const color = complexityToColor(undefined as any)
        expect(color).toBe('#94a3b8')
      })

      it('should return gray for null', () => {
        const color = complexityToColor(null as any)
        expect(color).toBe('#94a3b8')
      })
    })

    describe('gradient transitions', () => {
      it('should produce smooth transition from green to yellow (0-30)', () => {
        const colors = [0, 10, 20, 30].map(complexityToColor)
        // Each color should be different
        const uniqueColors = new Set(colors)
        expect(uniqueColors.size).toBe(colors.length)
      })

      it('should produce smooth transition from yellow to red (30-60)', () => {
        const colors = [30, 40, 50, 60].map(complexityToColor)
        // Each color should be different
        const uniqueColors = new Set(colors)
        expect(uniqueColors.size).toBe(colors.length)
      })

      it('should produce smooth transition in red range (60-100)', () => {
        const colors = [60, 80, 100].map(complexityToColor)
        // Each color should be different
        const uniqueColors = new Set(colors)
        expect(uniqueColors.size).toBe(colors.length)
      })
    })

    describe('color format', () => {
      it('should always return valid hex color format', () => {
        const testScores = [0, 15, 30, 45, 60, 80, 100, -10, 150]
        testScores.forEach(score => {
          const color = complexityToColor(score)
          expect(color).toMatch(/^#[0-9a-f]{6}$/i)
        })
      })
    })
  })

  describe('getComplexityGradient', () => {
    it('should return array of gradient stops', () => {
      const gradient = getComplexityGradient()
      expect(Array.isArray(gradient)).toBe(true)
      expect(gradient.length).toBeGreaterThan(0)
    })

    it('should have correct structure for each stop', () => {
      const gradient = getComplexityGradient()
      gradient.forEach(stop => {
        expect(stop).toHaveProperty('score')
        expect(stop).toHaveProperty('color')
        expect(stop).toHaveProperty('label')
        expect(typeof stop.score).toBe('number')
        expect(typeof stop.color).toBe('string')
        expect(typeof stop.label).toBe('string')
      })
    })

    it('should have scores in ascending order', () => {
      const gradient = getComplexityGradient()
      for (let i = 1; i < gradient.length; i++) {
        expect(gradient[i].score).toBeGreaterThanOrEqual(gradient[i - 1].score)
      }
    })

    it('should have valid hex colors', () => {
      const gradient = getComplexityGradient()
      gradient.forEach(stop => {
        expect(stop.color).toMatch(/^#[0-9a-f]{6}$/i)
      })
    })

    it('should include key thresholds (0, 30, 60, 100)', () => {
      const gradient = getComplexityGradient()
      const scores = gradient.map(stop => stop.score)
      expect(scores).toContain(0)
      expect(scores).toContain(30)
      expect(scores).toContain(60)
      expect(scores).toContain(100)
    })

    it('should have descriptive labels for key stops', () => {
      const gradient = getComplexityGradient()
      const labeled = gradient.filter(stop => stop.label.length > 0)
      expect(labeled.length).toBeGreaterThan(0)

      // Check that key labels exist
      const labels = labeled.map(stop => stop.label)
      expect(labels).toContain('Low')
      expect(labels).toContain('Medium')
      expect(labels).toContain('High')
    })
  })

  describe('getComplexityLabel', () => {
    describe('valid complexity scores', () => {
      it('should return "Low" for score 0', () => {
        expect(getComplexityLabel(0)).toBe('Low')
      })

      it('should return "Low" for score < 30', () => {
        expect(getComplexityLabel(10)).toBe('Low')
        expect(getComplexityLabel(29)).toBe('Low')
      })

      it('should return "Medium" for score 30', () => {
        expect(getComplexityLabel(30)).toBe('Medium')
      })

      it('should return "Medium" for score 30 <= x < 60', () => {
        expect(getComplexityLabel(40)).toBe('Medium')
        expect(getComplexityLabel(59)).toBe('Medium')
      })

      it('should return "High" for score 60', () => {
        expect(getComplexityLabel(60)).toBe('High')
      })

      it('should return "High" for score 60 <= x < 80', () => {
        expect(getComplexityLabel(70)).toBe('High')
        expect(getComplexityLabel(79)).toBe('High')
      })

      it('should return "Very High" for score 80', () => {
        expect(getComplexityLabel(80)).toBe('Very High')
      })

      it('should return "Very High" for score >= 80', () => {
        expect(getComplexityLabel(90)).toBe('Very High')
        expect(getComplexityLabel(100)).toBe('Very High')
      })
    })

    describe('edge cases', () => {
      it('should clamp negative values and return "Low"', () => {
        expect(getComplexityLabel(-10)).toBe('Low')
        expect(getComplexityLabel(-100)).toBe('Low')
      })

      it('should clamp values above 100 and return "Very High"', () => {
        expect(getComplexityLabel(150)).toBe('Very High')
        expect(getComplexityLabel(1000)).toBe('Very High')
      })

      it('should return "Unknown" for NaN', () => {
        expect(getComplexityLabel(NaN)).toBe('Unknown')
      })

      it('should return "Unknown" for non-number values', () => {
        expect(getComplexityLabel('invalid' as any)).toBe('Unknown')
        expect(getComplexityLabel(undefined as any)).toBe('Unknown')
        expect(getComplexityLabel(null as any)).toBe('Unknown')
      })
    })

    describe('boundary values', () => {
      it('should handle boundary at 30 correctly', () => {
        expect(getComplexityLabel(29.9)).toBe('Low')
        expect(getComplexityLabel(30)).toBe('Medium')
        expect(getComplexityLabel(30.1)).toBe('Medium')
      })

      it('should handle boundary at 60 correctly', () => {
        expect(getComplexityLabel(59.9)).toBe('Medium')
        expect(getComplexityLabel(60)).toBe('High')
        expect(getComplexityLabel(60.1)).toBe('High')
      })

      it('should handle boundary at 80 correctly', () => {
        expect(getComplexityLabel(79.9)).toBe('High')
        expect(getComplexityLabel(80)).toBe('Very High')
        expect(getComplexityLabel(80.1)).toBe('Very High')
      })
    })
  })
})
