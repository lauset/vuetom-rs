use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jint, jlong};
use jni::JNIEnv;

struct Counter {
    count: i32,
    callback: GlobalRef,
}

impl Counter {
    pub fn new(callback: GlobalRef) -> Counter {
        Counter {
            count: 0,
            callback: callback,
        }
    }

    pub fn increment(&mut self, env: &mut JNIEnv) {
        self.count = self.count + 1;
        env.call_method(
            &self.callback,
            "counterCallback",
            "(I)V",
            &[self.count.into()],
        )
        .unwrap();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_counterNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let counter = Counter::new(global_ref);

    Box::into_raw(Box::new(counter)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_counterIncrement(
    mut env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let counter = &mut *(counter_ptr as *mut Counter);

    counter.increment(&mut env);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_github_lauset_vuetomx_rs_RustJNI_counterDestroy(
    _env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(counter_ptr as *mut Counter);
}
