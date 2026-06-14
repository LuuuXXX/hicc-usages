CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/namespace_nested_make

$(OUT): namespace_nested.cpp main.cpp namespace_nested.h
	$(CXX) $(CXXFLAGS) namespace_nested.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
