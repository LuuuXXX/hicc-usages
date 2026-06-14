#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. enum_class.cpp main.cpp -o /tmp/enum_class_standalone
/tmp/enum_class_standalone
