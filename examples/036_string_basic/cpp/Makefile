CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/string_basic_make

$(OUT): string_basic.cpp main.cpp string_basic.h
	$(CXX) $(CXXFLAGS) string_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
