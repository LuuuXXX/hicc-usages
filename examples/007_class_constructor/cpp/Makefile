NAME := class_constructor
CXX ?= clang++
CXXFLAGS := -std=c++17 -O2 -fPIC -Wall -Iinclude
SRCS := $(wildcard src/*.cpp)
OBJS := $(patsubst src/%.cpp,build/%.o,$(SRCS))

all: build/lib$(NAME).a

build/lib$(NAME).a: $(OBJS)
	@mkdir -p build
	$(AR) rcs $@ $^

build/%.o: src/%.cpp
	@mkdir -p build
	$(CXX) $(CXXFLAGS) -c $< -o $@

test: all
	$(CXX) $(CXXFLAGS) test/main.cpp -Lbuild -l$(NAME) -o build/test_$(NAME) && ./build/test_$(NAME)

clean:
	rm -rf build

.PHONY: all test clean
