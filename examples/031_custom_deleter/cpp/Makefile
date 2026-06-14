CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/custom_deleter_make

$(OUT): custom_deleter.cpp main.cpp custom_deleter.h
	$(CXX) $(CXXFLAGS) custom_deleter.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
