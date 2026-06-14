CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/union_basic_make

$(OUT): union_basic.cpp main.cpp union_basic.h
	$(CXX) $(CXXFLAGS) union_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
