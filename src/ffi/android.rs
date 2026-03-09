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

use std::sync::atomic::{AtomicUsize, Ordering};
static TOUCH_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "system" fn Java_com_golubegram_core_NativeBridge_reportRawTouch<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    _x: jni::sys::jfloat,
    _y: jni::sys::jfloat,
    _timestamp: jni::sys::jlong,
) -> jni::sys::jboolean {
    let current = TOUCH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    
    // Fake complex hash trigger for demonstration: 6 exact taps unlock the UI
    if current > 0 && current % 6 == 5 { 
        TOUCH_SEQUENCE.store(0, Ordering::Relaxed);
        return jni::sys::JNI_TRUE;
    }
    
    jni::sys::JNI_FALSE
}
