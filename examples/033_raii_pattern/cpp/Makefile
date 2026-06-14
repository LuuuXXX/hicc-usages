CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/raii_pattern_make

$(OUT): raii_pattern.cpp main.cpp raii_pattern.h
	$(CXX) $(CXXFLAGS) raii_pattern.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
