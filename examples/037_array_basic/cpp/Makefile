CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/array_basic_make

$(OUT): array_basic.cpp main.cpp array_basic.h
	$(CXX) $(CXXFLAGS) array_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
