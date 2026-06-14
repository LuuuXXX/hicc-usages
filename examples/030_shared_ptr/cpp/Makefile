CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/shared_ptr_make

$(OUT): shared_ptr.cpp main.cpp shared_ptr.h
	$(CXX) $(CXXFLAGS) shared_ptr.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
