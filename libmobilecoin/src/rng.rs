use crate::common::*;
use mc_util_ffi::*;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use rand_core::RngCore;
use std::sync::Mutex;
use std::convert::TryInto;


pub type McChaCha20Rng = ChaCha20Rng;


impl_into_ffi!(Mutex<McChaCha20Rng>);
impl_into_ffi!(FfiOwnedPtr<McU128>);

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_create_with_long(long_val: u64) -> FfiOptOwnedPtr<Mutex<McChaCha20Rng>> {
    ffi_boundary(|| {
        Mutex::new(McChaCha20Rng::seed_from_u64(long_val))
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_create_with_bytes(bytes: FfiRefPtr<McBuffer>) -> FfiOptOwnedPtr<Mutex<McChaCha20Rng>> {
    ffi_boundary(|| {
        let bytes: [u8; 32] = bytes.as_slice().try_into().expect("seed size must be 32 bytes");
        Mutex::new(McChaCha20Rng::from_seed(bytes))
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_get_word_pos(
    chacha20_rng_mtx: FfiMutPtr<Mutex<McChaCha20Rng>>,
    out_word_pos: FfiMutPtr<McMutableBuffer>,
) {
    ffi_boundary(|| {
        let word_pos = chacha20_rng_mtx.lock().unwrap().get_word_pos();
        let mc_u128 = McU128::from_u128(word_pos);

        let out_word_pos = out_word_pos
            .into_mut()
            .as_slice_mut_of_len(16)
            .expect("word_pos length is not exaclty 16 bytes");

        out_word_pos.copy_from_slice(&mc_u128.bytes);
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_set_word_pos(chacha20_rng_mtx: FfiMutPtr<Mutex<McChaCha20Rng>>, mc_u128: FfiRefPtr<McBuffer>) {
    ffi_boundary(|| {
        let mc_u128 = McU128 {
            bytes: mc_u128.as_slice().try_into().expect("word_pos length is not exaclty 16 bytes")
        };
        let word_pos = mc_u128.to_u128();
        chacha20_rng_mtx.lock().unwrap().set_word_pos(word_pos);
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