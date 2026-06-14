#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. union_basic.cpp main.cpp -o /tmp/union_basic_standalone
/tmp/union_basic_standalone
