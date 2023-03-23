use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use std::{sync::mpsc, thread, time::Duration};

#[no_mangle]
pub extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_factFn(
    mut env: JNIEnv,
    _class: JClass,
    n: jint,
    callback: JObject,
) {
    let i = n as i32;
    let res: jint = (2..i + 1).product();

    env.call_method(callback, "factCallback", "(I)V", &[res.into()])
        .unwrap();
}

#[no_mangle]
pub extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_asyncFn(
    env: JNIEnv,
    _class: JClass,
    count: jint,
    sleep: jint,
    callback: JObject,
) {
    // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
    // functions in other threads, we must first obtain the `JavaVM` interface
    // which, unlike `JNIEnv` is `Send`.
    let jvm = env.get_java_vm().unwrap();

    // We need to obtain global reference to the `callback` object before sending
    // it to the thread, to prevent it from being collected by the GC.
    let callback = env.new_global_ref(callback).unwrap();

    // Use channel to prevent the Java program to finish before the thread
    // has chance to start.
    let (tx, rx) = mpsc::channel();

    let s = sleep as u64;

    let _ = thread::spawn(move || {
        // Signal that the thread has started.
        tx.send(()).unwrap();

        // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
        let mut env = jvm.attach_current_thread().unwrap();

        for i in 0..count + 1 {
            let progress = (i * 1) as jint;
            // Now we can use all available `JNIEnv` functionality normally.
            env.call_method(&callback, "asyncCallback", "(I)V", &[progress.into()])
                .unwrap();
            thread::sleep(Duration::from_millis(s));
        }

        // The current thread is detached automatically when `env` goes out of scope.
    });

    // Wait until the thread has started.
    rx.recv().unwrap();
}
