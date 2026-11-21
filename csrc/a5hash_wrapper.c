#include "a5hash.h"
#include <stdint.h>
#include <stddef.h>

// SMHasher-compatible: a5hash-128.64
uint64_t a5hash_128_64(const void* data, size_t len, uint64_t seed) {
    uint64_t high;
    uint64_t low = a5hash128(data, len, seed, &high);
    return low;  // lower 64 bits
}
