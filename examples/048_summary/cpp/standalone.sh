#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. summary.cpp main.cpp -o /tmp/summary_standalone
/tmp/summary_standalone
