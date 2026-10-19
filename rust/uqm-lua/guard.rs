// SPDX-License-Identifier: GPL-2.0-or-later
// Shared by the generated HD engine and the independent Lua boundary oracle.
extern "C" {
    pub fn uqm_lua_failed() -> ::core::ffi::c_int;
}

/// Return normally through Rust callers before the outer C trampoline throws.
/// The generator supplies an explicit, type-checked failure value: unit, zero,
/// false, a null pointer, None, or a field-by-field C aggregate construction.
/// No Rust reference, nonzero integer or Rust enum is fabricated from bytes.
macro_rules! uqm_lua_checked {
    ($operation:expr, $failure:expr) => {{
        let result = $operation;
        if unsafe { crate::uqm_lua_guard::uqm_lua_failed() } != 0 {
            return $failure;
        }
        result
    }};
}
