CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/template_specialization_make

$(OUT): template_specialization.cpp main.cpp template_specialization.h
	$(CXX) $(CXXFLAGS) template_specialization.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
