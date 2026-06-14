#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::variadic_template {

template<typename... Args>
int sum_all(Args... args) { return (... + args); }

template<typename... Args>
int count_all(Args...) { return sizeof...(Args); }

template<typename... Args>
int max_all(Args... args) {
    int arr[] = { args... };
    int m = arr[0];
    for (std::size_t i = 1; i < sizeof...(Args); ++i) {
        if (arr[i] > m) m = arr[i];
    }
    return m;
}

}
