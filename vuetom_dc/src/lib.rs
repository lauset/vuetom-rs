use jni::objects::*;
use jni::sys::jint;
use jni::JNIEnv;

mod func;
mod util;

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_init(
    env: JNIEnv,
    _class: JClass,
) {
    println!("lib vuetom_dc inited.");
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_divInt(
    mut env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    if b == 0 {
        env.throw_new("Ljava/lang/Exception;", "divide zero")
            .expect("throw");
        0
    } else {
        a / b
    }
}
