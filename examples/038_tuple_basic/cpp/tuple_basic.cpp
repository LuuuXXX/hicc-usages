#include "tuple_basic.h"

namespace tuple_basic_ns {

std::unique_ptr<Triple> make_triple(int id, const std::string& name, double score) {
    return std::unique_ptr<Triple>(new Triple(id, name, score));
}

int triple_id(const Triple& t) {
    return std::get<0>(t);
}

std::string triple_name(const Triple& t) {
    return std::get<1>(t);
}

double triple_score(const Triple& t) {
    return std::get<2>(t);
}

void set_id(Triple& t, int id) {
    std::get<0>(t) = id;
}

void set_score(Triple& t, double score) {
    std::get<2>(t) = score;
}

int tuple_basic_anchor() { return 38; }

} // namespace tuple_basic_ns
