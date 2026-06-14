#pragma once

#include <string>

// Override: derived class provides its own impl of a base virtual method.
// Rust side just sees the derived class — override is transparent to FFI.

class Logger {
public:
    virtual ~Logger() = default;
    virtual std::string level() const = 0;
    std::string format(const std::string& msg) const {
        return "[" + level() + "] " + msg;
    }
};

class InfoLogger : public Logger {
public:
    std::string level() const override { return "INFO"; }
};

class ErrorLogger : public Logger {
public:
    std::string level() const override { return "ERROR"; }
};

InfoLogger*  info_logger_new();
ErrorLogger* error_logger_new();
void         logger_free_info(InfoLogger* l);
void         logger_free_error(ErrorLogger* l);
