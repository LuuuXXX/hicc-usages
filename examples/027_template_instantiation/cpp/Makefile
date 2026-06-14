CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/template_instantiation_make

$(OUT): template_instantiation.cpp main.cpp template_instantiation.h
	$(CXX) $(CXXFLAGS) template_instantiation.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
