"""Prebuilt native hash functions for SpeeduPy.

Three hashes, each a function of a bytes-like object:

- ``a5hash_128_64(data, seed=0)``: lower 64 bits of a5hash-128;
- ``museair_bfast_64(data, seed=0)``: MuseAir BFast, 64-bit;
- ``meowhash_32(data)``: lower 32 bits of MeowHash 0.5 with its default seed.
  Available on x86_64 with AES-NI only: ``HAS_MEOWHASH`` says whether it runs
  here, and the function raises ``RuntimeError`` where it does not.

The values are the ones the original ``ctypes`` wrappers of SpeeduPy returned,
computed from the same pinned sources (see ``docs/native-hashes.md`` in the
SpeeduPy repository).
"""

from speedupy_native._speedupy_native import (
    HAS_MEOWHASH,
    PLATFORM,
    __version__,
    a5hash_128_64,
    meowhash_32,
    museair_bfast_64,
)

__all__ = [
    "HAS_MEOWHASH",
    "PLATFORM",
    "__version__",
    "a5hash_128_64",
    "meowhash_32",
    "museair_bfast_64",
]
