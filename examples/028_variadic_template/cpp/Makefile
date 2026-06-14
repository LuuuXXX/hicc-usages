CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/variadic_template_make

$(OUT): variadic_template.cpp main.cpp variadic_template.h
	$(CXX) $(CXXFLAGS) variadic_template.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
