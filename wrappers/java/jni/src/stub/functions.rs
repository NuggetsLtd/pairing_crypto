use jni::{objects::JObject, sys::jlong, JNIEnv};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_pairing_1crypto_Stub_add(
    _: JNIEnv,
    _: JObject,
    a: jlong,
    b: jlong,
) -> jlong {
    pairing_crypto_c::stub::functions::harry_add(a, b)
}
