CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/unique_ptr_make

$(OUT): unique_ptr.cpp main.cpp unique_ptr.h
	$(CXX) $(CXXFLAGS) unique_ptr.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
