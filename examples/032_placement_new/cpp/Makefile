CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/placement_new_make

$(OUT): placement_new.cpp main.cpp placement_new.h
	$(CXX) $(CXXFLAGS) placement_new.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
