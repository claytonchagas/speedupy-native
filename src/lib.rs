//! `speedupy_native._speedupy_native`: the three native hashes of SpeeduPy.
//!
//! Each function takes a `bytes` object (what SpeeduPy's serializers produce)
//! and returns the same integer the `ctypes` wrappers of the original `.so`
//! files returned:
//!
//! - `a5hash_128_64(data, seed=0)`: lower 64 bits of a5hash-128 (SMHasher's
//!   a5hash-128.64), through `a5hash_wrapper.c`;
//! - `museair_bfast_64(data, seed=0)`: MuseAir BFast 64-bit, the same call
//!   the fork's `museair_bfast_64` wrapper makes;
//! - `meowhash_32(data)`: lower 32 bits of MeowHash 0.5 with the default seed,
//!   through `meow_wrapper.cpp` (x86_64 with AES-NI only).

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::os::raw::c_void;

extern "C" {
    fn a5hash_128_64(data: *const c_void, len: usize, seed: u64) -> u64;
}

#[cfg(has_meowhash)]
extern "C" {
    fn MeowHash_32(seed: *mut c_void, len: u64, data: *mut c_void) -> u32;
    fn GetMeowDefaultSeed() -> *mut c_void;
}

/// a5hash-128.64: the lower 64 bits of a5hash-128 of `data` with `seed`.
#[pyfunction]
#[pyo3(name = "a5hash_128_64", signature = (data, seed = 0))]
fn a5hash_128_64_py(data: &[u8], seed: u64) -> u64 {
    unsafe { a5hash_128_64(data.as_ptr() as *const c_void, data.len(), seed) }
}

/// MuseAir BFast, 64-bit, of `data` with `seed`.
#[pyfunction]
#[pyo3(name = "museair_bfast_64", signature = (data, seed = 0))]
fn museair_bfast_64_py(data: &[u8], seed: u64) -> u64 {
    museair::bfast::hash(data, seed)
}

/// Whether MeowHash can run here: built for x86_64 and the CPU has AES-NI.
fn meowhash_available() -> bool {
    #[cfg(has_meowhash)]
    {
        std::arch::is_x86_feature_detected!("aes") && std::arch::is_x86_feature_detected!("sse4.1")
    }
    #[cfg(not(has_meowhash))]
    {
        false
    }
}

fn platform() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

/// MeowHash 0.5, lower 32 bits, default seed. Raises `RuntimeError` where
/// MeowHash is unavailable (not x86_64, or no AES-NI on this CPU).
#[pyfunction]
#[pyo3(name = "meowhash_32")]
fn meowhash_32_py(data: &[u8]) -> PyResult<u32> {
    if !meowhash_available() {
        return Err(PyRuntimeError::new_err(format!(
            "MeowHash is unavailable on {}: it needs an x86_64 CPU with AES-NI; \
             the other hashes (md5, murmur, xxhash, a5hash, museair) work here",
            platform()
        )));
    }
    #[cfg(has_meowhash)]
    {
        Ok(unsafe {
            MeowHash_32(
                GetMeowDefaultSeed(),
                data.len() as u64,
                data.as_ptr() as *mut c_void,
            )
        })
    }
    #[cfg(not(has_meowhash))]
    {
        let _ = data;
        unreachable!()
    }
}

#[pymodule]
fn _speedupy_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("HAS_MEOWHASH", meowhash_available())?;
    m.add("PLATFORM", platform())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(a5hash_128_64_py, m)?)?;
    m.add_function(wrap_pyfunction!(museair_bfast_64_py, m)?)?;
    m.add_function(wrap_pyfunction!(meowhash_32_py, m)?)?;
    Ok(())
}
