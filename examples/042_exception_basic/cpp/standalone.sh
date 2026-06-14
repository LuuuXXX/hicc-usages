#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. exception_basic.cpp main.cpp -o /tmp/exception_basic_standalone
/tmp/exception_basic_standalone
