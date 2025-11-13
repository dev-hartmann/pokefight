#!/bin/bash

# Test script for /api/tournament endpoint

# Default values
HOST="${HOST:-localhost}"
PORT="${PORT:-3000}"
URL="http://${HOST}:${PORT}/api/tournament"

# Test data with valid Pokemon names
JSON_DATA='{
  "names": ["Ash", "Gary", "Misty", "Brock"],
  "chore": "Clean the dishes",
  "title": "Weekly Pokemon Tournament"
}'

echo "Testing tournament endpoint at: $URL"
echo "Request payload:"
echo "$JSON_DATA" | jq '.'
echo ""
echo "Response:"

curl -X POST "$URL" \
  -H "Content-Type: application/json" \
  -d "$JSON_DATA" \
  -w "\n\nHTTP Status: %{http_code}\n" \
