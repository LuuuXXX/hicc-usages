#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. functional_bind.cpp main.cpp -o /tmp/functional_bind_standalone
/tmp/functional_bind_standalone
