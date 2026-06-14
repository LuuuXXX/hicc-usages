CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/noexcept_basic_make

$(OUT): noexcept_basic.cpp main.cpp noexcept_basic.h
	$(CXX) $(CXXFLAGS) noexcept_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
