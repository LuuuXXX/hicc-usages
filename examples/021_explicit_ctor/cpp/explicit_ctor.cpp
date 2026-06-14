#include "explicit_ctor.h"

Celsius Fahrenheit::to_celsius() const {
    return Celsius((value_ - 32.0) * 5.0 / 9.0);
}

Celsius    convert_to_celsius(const Fahrenheit& f) { return f.to_celsius(); }

Celsius*    celsius_new(double v)    { return new Celsius(v); }
Fahrenheit* fahrenheit_new(double v) { return new Fahrenheit(v); }
void        celsius_free(Celsius* c)    { delete c; }
void        fahrenheit_free(Fahrenheit* f) { delete f; }
