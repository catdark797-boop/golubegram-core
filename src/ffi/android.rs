use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

/// Android specific JNI exports

#[no_mangle]
pub extern "system" fn Java_com_golubegram_core_NativeBridge_initCore<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    crate::init_core();
}

#[no_mangle]
pub extern "system" fn Java_com_golubegram_core_NativeBridge_panicLock<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
) {
    crate::crypto::hidden::panic_lock();
}

#[no_mangle]
pub extern "system" fn Java_com_golubegram_core_NativeBridge_topUpCredits<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    amount: jni::sys::jlong,
) {
    crate::billing::GLOBAL_BILLING.top_up(amount as u64);
}
