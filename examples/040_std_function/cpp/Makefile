CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/std_function_make

$(OUT): std_function.cpp main.cpp std_function.h
	$(CXX) $(CXXFLAGS) std_function.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
