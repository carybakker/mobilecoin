use crate::{common::*};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use rand_core::RngCore;
use std::sync::Mutex;

use mc_util_ffi::*;

pub type McChaCha20Rng = ChaCha20Rng;
impl_into_ffi!(Mutex<McChaCha20Rng>);
    
#[no_mangle]
pub extern "C" fn mc_chacha20_rng_create_with_long(long_val: u64) -> FfiOptOwnedPtr<Mutex<McChaCha20Rng>> {
    ffi_boundary(|| {
        Mutex::new(McChaCha20Rng::seed_from_u64(long_val))
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_next_long(chacha20_rng_mtx: FfiMutPtr<Mutex<McChaCha20Rng>>) -> u64 {
    ffi_boundary(|| {
        let next = chacha20_rng_mtx.lock().unwrap().next_u64();
        next
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_free(chacha20_rng_mtx: FfiOptOwnedPtr<Mutex<McChaCha20Rng>>) {
    ffi_boundary(|| {
        let _ = chacha20_rng_mtx;
    })
}