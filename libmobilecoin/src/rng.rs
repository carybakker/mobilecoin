use crate::{common::*};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use rand_core::RngCore;

use mc_util_ffi::*;

pub type McChaCha20Rng = ChaCha20Rng;
impl_into_ffi!(McChaCha20Rng);
    
#[no_mangle]
pub extern "C" fn mc_chacha20_rng_create_with_long(long_val: u64) -> FfiOptOwnedPtr<McChaCha20Rng> {
    ffi_boundary(|| {
        let rng = McChaCha20Rng::seed_from_u64(long_val);
        return rng;
    })
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_next_int(mut chacha20_rng: FfiMutPtr<McChaCha20Rng>) -> u32 {
    return chacha20_rng.next_u32()
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_next_long(mut chacha20_rng: FfiMutPtr<McChaCha20Rng>) -> u64 {
    return chacha20_rng.next_u64()
}

#[no_mangle]
pub extern "C" fn mc_chacha20_rng_free(chacha20_rng: FfiOptOwnedPtr<McChaCha20Rng>) {
    ffi_boundary(|| {
        let _ = chacha20_rng;
    })
}



// /********************************************************************
//  * ChaCha20Rng
//  */
// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_seed_1from_1long(
//     env: JNIEnv,
//     _obj: JObject,
//     seed: jlong,
// ) -> jobject {
//     jni_ffi_call_or(
//         || Ok(JObject::null().into_inner()),
//         &env,
//         |env| {
//             let rng = ChaCha20Rng::seed_from_u64(seed as u64);
//             let mbox = Box::new(Mutex::new(rng));
//             let ptr: *mut Mutex<ChaCha20Rng> = Box::into_raw(mbox);
//             Ok(env
//                 .new_object(
//                     "com/mobilecoin/lib/ChaCha20Rng",
//                     "(J)V",
//                     &[
//                         jni::objects::JValue::Long(ptr as jlong)
//                     ],
//                 )?
//                 .into_inner())
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_seed_1from_1bytes(
//     env: JNIEnv,
//     _obj: JObject,
//     seed: jbyteArray,
// ) -> jobject {
//     jni_ffi_call_or(
//         || Ok(JObject::null().into_inner()),
//         &env,
//         |env| {
//             let seed_bytes = env.convert_byte_array(seed)?
//                 .try_into().unwrap();
//             let rng = ChaCha20Rng::from_seed(seed_bytes);
//             let mbox = Box::new(Mutex::new(rng));
//             let ptr: *mut Mutex<ChaCha20Rng> = Box::into_raw(mbox);
//             Ok(env
//                 .new_object(
//                     "com/mobilecoin/lib/ChaCha20Rng",
//                     "(J)V",
//                     &[
//                         jni::objects::JValue::Long(ptr as jlong)
//                     ],
//                 )?
//                 .into_inner())
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_next_1int(
//     env: JNIEnv,
//     obj: JObject,
// ) -> jint {
//     jni_ffi_call_or(
//         || Ok(0),
//         &env,
//         |env| {
//             let mut rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//             Ok(rng.next_u32() as jint)
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_next_1long(
//     env: JNIEnv,
//     obj: JObject,
// ) -> jlong {
//     jni_ffi_call_or(
//         || Ok(0),
//         &env,
//         |env| {
//             let mut rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//             Ok(rng.next_u64() as jlong)
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_next_1bytes(
//     env: JNIEnv,
//     obj: JObject,
//     length: jint,
// ) -> jbyteArray {
//     jni_ffi_call_or(
//         || Ok(JObject::null().into_inner()),
//         &env,
//         |env| {
//             let mut rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//             let mut bytes = vec![0; length as usize];
//             rng.fill_bytes(&mut bytes);
//             Ok(env.byte_array_from_slice(&bytes)?)
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_get_1seed(
//     env: JNIEnv,
//     obj: JObject,
// ) -> jbyteArray {
//     jni_ffi_call_or(
//         || Ok(JObject::null().into_inner()),
//         &env,
//         |env| {
//             let rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//             Ok(env.byte_array_from_slice(&rng.get_seed())?)
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_get_1word_1pos(
//     env: JNIEnv,
//     obj: JObject,
// ) -> jobject {
//     jni_ffi_call_or(
//         || Ok(JObject::null().into_inner()),
//         &env,
//         |env| {
//             let rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//             // Java is always big endian
//             let word_pos_bytes = rng.get_word_pos().to_be_bytes();
//             let word_pos = env.new_object(
//                 "java/math/BigInteger",
//                 "(I[B)V",
//                 &[
//                     jni::objects::JValue::Int(1),
//                     env.byte_array_from_slice(&word_pos_bytes)?
//                         .into(),
//                 ],
//             )?;
//             Ok(word_pos.into_inner())
//         },
//     )
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_set_1word_1pos(
//     env: JNIEnv,
//     obj: JObject,
//     word_pos_bytes: jbyteArray,
// ) {
//     jni_ffi_call(&env, |env| {
//         let mut rng: MutexGuard<ChaCha20Rng> = env.get_rust_field(obj, RUST_OBJ_FIELD)?;
//         let word_pos: [u8; 16] = env.convert_byte_array(word_pos_bytes)?
//             .try_into().unwrap();
//         // Java is always big endian
//         rng.set_word_pos(u128::from_be_bytes(word_pos));
//         Ok(())
//     })
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_com_mobilecoin_lib_ChaCha20Rng_finalize_1jni(
//     env: JNIEnv,
//     obj: JObject,
// ) {
//     jni_ffi_call(&env, |env| {
//         let _: ChaCha20Rng = env.take_rust_field(obj, RUST_OBJ_FIELD)?;
//         Ok(())
//     })
// }
