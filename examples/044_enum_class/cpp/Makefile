CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/enum_class_make

$(OUT): enum_class.cpp main.cpp enum_class.h
	$(CXX) $(CXXFLAGS) enum_class.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
