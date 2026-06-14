#include "placement_new.h"

namespace placement_new_ns {

Payload* place_payload(Buffer& buf, int value) {
    return new (buf.raw()) Payload(value);
}

Payload* place_payload_raw(void* raw, int value) {
    return new (raw) Payload(value);
}

void destroy_payload(Payload* p) {
    if (p) p->~Payload();
}

int placement_new_anchor() { return 32; }
}
