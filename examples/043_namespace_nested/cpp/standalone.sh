#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. namespace_nested.cpp main.cpp -o /tmp/namespace_nested_standalone
/tmp/namespace_nested_standalone
