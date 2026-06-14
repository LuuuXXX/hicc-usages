CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/exception_basic_make

$(OUT): exception_basic.cpp main.cpp exception_basic.h
	$(CXX) $(CXXFLAGS) exception_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
