# Makefile template — mirrors standalone.sh
# Replace class_volatile and class_volatile.cpp placeholders when scaffolding.
NAME := class_volatile
CXX ?= g++
CXX_FLAGS ?= -O2 -std=c++17 -Wall -Wextra -fPIC
SOURCES := class_volatile.cpp
OBJECTS := $(addprefix build/,$(SOURCES:.cpp=.o))

HERE := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

.PHONY: all clean demo

all: build/lib$(NAME).a

build:
	mkdir -p build

build/%.o: %.cpp | build
	$(CXX) $(CXX_FLAGS) -I$(HERE) -c $< -o $@

build/lib$(NAME).a: $(OBJECTS)
	ar rcs $@ $^

demo: build/lib$(NAME).a
	$(CXX) $(CXX_FLAGS) -I$(HERE) main.cpp -Lbuild -l$(NAME) -o build/$(NAME)_demo

clean:
	rm -rf build
