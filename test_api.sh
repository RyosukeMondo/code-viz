#!/bin/bash
# Test script to verify backend API response structure

echo "Testing code-viz-web API..."
echo "============================"
echo

# Test /api/health
echo "1. Testing /api/health endpoint"
curl -s http://localhost:8080/api/health | jq '.'
echo
echo

# Test /api/analyze with small directory
echo "2. Testing /api/analyze with /tmp"
RESPONSE=$(curl -s -X POST http://localhost:8080/api/analyze \
  -H "Content-Type: application/json" \
  -d '{"path": "/tmp"}')

echo "Root node structure:"
echo "$RESPONSE" | jq '{
  id, name, path, loc, complexity, type,
  childrenCount: (.children | length),
  firstChild: .children[0] | {name, type, loc}
}'

echo
echo "Checking field names match TypeScript expectations:"
echo "$RESPONSE" | jq 'if .type then "✅ type field exists" else "❌ type field missing (found: \(.nodeType // "null"))" end'
echo "$RESPONSE" | jq 'if .name then "✅ name field exists" else "❌ name field missing" end'
echo "$RESPONSE" | jq 'if .path != null then "✅ path field exists" else "❌ path field missing" end'
echo "$RESPONSE" | jq 'if .loc then "✅ loc field exists" else "❌ loc field missing" end'
echo "$RESPONSE" | jq 'if .children then "✅ children field exists" else "❌ children field missing" end'

echo
echo "3. Sample child node:"
echo "$RESPONSE" | jq '.children[0]'

echo
echo "Done!"
