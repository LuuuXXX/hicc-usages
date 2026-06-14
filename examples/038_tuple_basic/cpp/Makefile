CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/tuple_basic_make

$(OUT): tuple_basic.cpp main.cpp tuple_basic.h
	$(CXX) $(CXXFLAGS) tuple_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
