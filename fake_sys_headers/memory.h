#pragma once

inline void* operator new(size_t size, void* where) {
    // ????
    return where;
}
