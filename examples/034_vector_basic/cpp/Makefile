CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/vector_basic_make

$(OUT): vector_basic.cpp main.cpp vector_basic.h
	$(CXX) $(CXXFLAGS) vector_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
