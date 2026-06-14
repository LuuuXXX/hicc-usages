CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/constexpr_basic_make

$(OUT): constexpr_basic.cpp main.cpp constexpr_basic.h
	$(CXX) $(CXXFLAGS) constexpr_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
