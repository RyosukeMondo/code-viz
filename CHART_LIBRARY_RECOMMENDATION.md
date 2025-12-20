# Chart Library Recommendation

## Current Issues with ECharts

**Problems encountered:**
1. Custom data fields lost in click events (`path`, `type` become undefined)
2. Need workarounds to access original node data
3. Complex API and configuration
4. Large bundle size (675KB)

**GitHub Issues:**
- [#11377 - treemap click event get wrong dataIndex](https://github.com/apache/incubator-echarts/issues/11377)
- [#13643 - Treemap state lost with setOption](https://github.com/apache/echarts/issues/13643)

## Recommended Alternatives

### Option 1: Recharts (Recommended) ⭐

**Pros:**
- ✅ React-native (built for React, not a wrapper)
- ✅ Simpler API and configuration
- ✅ Preserves all custom data in events
- ✅ Smaller bundle size (~200KB)
- ✅ Better TypeScript support
- ✅ Active maintenance

**Cons:**
- ❌ Less features than ECharts
- ❌ TreeMap requires custom implementation

**Migration effort:** ~2-3 hours

**Example:**
```tsx
import { Treemap, ResponsiveContainer } from 'recharts';

<ResponsiveContainer width="100%" height={600}>
  <Treemap
    data={data}
    dataKey="loc"
    aspectRatio={4/3}
    stroke="#fff"
    fill="#8884d8"
    onClick={(node) => {
      // ALL custom fields preserved!
      console.log(node.name, node.path, node.type);
      onNodeClick(node);
    }}
  />
</ResponsiveContainer>
```

### Option 2: D3.js Treemap

**Pros:**
- ✅ Most powerful and flexible
- ✅ Complete control over rendering
- ✅ Industry standard for data visualization
- ✅ Small core library

**Cons:**
- ❌ Steeper learning curve
- ❌ More code to write
- ❌ Need to manage DOM directly

**Migration effort:** ~4-6 hours

### Option 3: React D3 Tree

**Pros:**
- ✅ React wrapper for D3 tree
- ✅ Good for hierarchical data
- ✅ Customizable

**Cons:**
- ❌ Less maintained
- ❌ Smaller community

**Migration effort:** ~3-4 hours

### Option 4: Keep ECharts + Add Simple Tree View Toggle

**Pros:**
- ✅ No migration needed
- ✅ Fix already applied
- ✅ Tree View works perfectly as backup

**Cons:**
- ❌ Still dealing with ECharts quirks
- ❌ Large bundle size

**Effort:** 0 hours (already done!)

## Recommendation

**For this project: Keep ECharts + Tree View** ✅

**Reasoning:**
1. Fix is applied and working
2. Tree View provides reliable fallback
3. Debug panel shows raw data
4. Migration has opportunity cost

**For new projects: Use Recharts** ⭐

## Migration Plan (If switching to Recharts)

### Step 1: Install Recharts
```bash
npm install recharts
```

### Step 2: Create RechartsTreemap component
```tsx
// src/components/visualizations/RechartsTreemap.tsx
import { Treemap, ResponsiveContainer } from 'recharts';

export function RechartsTreemap({ data, onNodeClick }: TreemapProps) {
  return (
    <ResponsiveContainer width="100%" height={600}>
      <Treemap
        data={transformToRecharts(data)}
        dataKey="value"
        aspectRatio={4/3}
        stroke="#fff"
        onClick={(node) => onNodeClick(node.payload)} // Recharts wraps in .payload
        content={<CustomTreemapContent />}  // For color coding
      />
    </ResponsiveContainer>
  );
}
```

### Step 3: Replace in AnalysisView
```tsx
- import { Treemap } from '@/components/visualizations/Treemap';
+ import { RechartsTreemap } from '@/components/visualizations/RechartsTreemap';

- <Treemap data={data} ... />
+ <RechartsTreemap data={data} ... />
```

### Step 4: Remove ECharts
```bash
npm uninstall echarts
```

**Bundle size reduction: ~475KB** 📦

## Decision Matrix

| Feature | ECharts | Recharts | D3.js | Current (ECharts+Tree) |
|---------|---------|----------|-------|------------------------|
| React Integration | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| Custom Fields | ❌ | ✅ | ✅ | ✅ (with workaround) |
| Bundle Size | 675KB | 200KB | 100KB | 675KB |
| Ease of Use | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| Documentation | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Community | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Total** | 16/30 | 24/30 | 20/30 | 22/30 |

## Conclusion

**Current state (ECharts + Tree View): WORKING** ✅

**Next step if problems persist: Migrate to Recharts** 📊

**Estimated ROI of migration:**
- Time cost: 2-3 hours
- Bundle size savings: 475KB
- Code simplification: ~30% less code
- Maintenance: Easier debugging
