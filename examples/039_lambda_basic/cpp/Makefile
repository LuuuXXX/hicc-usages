CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/lambda_basic_make

$(OUT): lambda_basic.cpp main.cpp lambda_basic.h
	$(CXX) $(CXXFLAGS) lambda_basic.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
