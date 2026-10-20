#!/usr/bin/env bash
set -euo pipefail

API_URL="${API_URL:-http://127.0.0.1:8080}"

printf 'Checking %s/health...\n' "$API_URL"
curl --fail --silent --show-error "$API_URL/health"
printf '\nOK\n'
