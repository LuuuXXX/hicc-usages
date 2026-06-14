CXX ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -I.
OUT ?= /tmp/functional_bind_make

$(OUT): functional_bind.cpp main.cpp functional_bind.h
	$(CXX) $(CXXFLAGS) functional_bind.cpp main.cpp -o $(OUT)

run: $(OUT)
	$(OUT)

clean:
	rm -f $(OUT)

.PHONY: run clean
