// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-NetworkOnMainThreadException"))]
__jni_bindgen! {
    /// public class [NetworkOnMainThreadException](https://developer.android.com/reference/android/os/NetworkOnMainThreadException.html)
    ///
    /// Required feature: android-os-NetworkOnMainThreadException
    public class NetworkOnMainThreadException ("android/os/NetworkOnMainThreadException") extends crate::java::lang::RuntimeException {

        /// [NetworkOnMainThreadException](https://developer.android.com/reference/android/os/NetworkOnMainThreadException.html#NetworkOnMainThreadException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::NetworkOnMainThreadException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/NetworkOnMainThreadException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/NetworkOnMainThreadException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
