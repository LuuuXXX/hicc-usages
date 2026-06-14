#include "unique_ptr.h"

std::unique_ptr<Widget> make_widget(int v) { return std::make_unique<Widget>(v); }
void widget_free(Widget* w) { /* unique_ptr deleter calls delete; here just delete */ delete w; }
