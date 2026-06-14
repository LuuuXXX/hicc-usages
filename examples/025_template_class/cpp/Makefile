CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/template_class_make

$(OUT): template_class.cpp main.cpp template_class.h
	$(CXX) $(CXXFLAGS) template_class.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
