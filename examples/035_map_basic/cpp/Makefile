CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/map_basic_make

$(OUT): map_basic.cpp main.cpp map_basic.h
	$(CXX) $(CXXFLAGS) map_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
