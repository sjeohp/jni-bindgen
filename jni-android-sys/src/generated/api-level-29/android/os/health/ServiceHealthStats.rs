// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-health-ServiceHealthStats"))]
__jni_bindgen! {
    /// public final class [ServiceHealthStats](https://developer.android.com/reference/android/os/health/ServiceHealthStats.html)
    ///
    /// Required feature: android-os-health-ServiceHealthStats
    public final class ServiceHealthStats ("android/os/health/ServiceHealthStats") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [ServiceHealthStats](https://developer.android.com/reference/android/os/health/ServiceHealthStats.html#ServiceHealthStats())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::health::ServiceHealthStats>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/health/ServiceHealthStats", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/health/ServiceHealthStats\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [MEASUREMENT_LAUNCH_COUNT](https://developer.android.com/reference/android/os/health/ServiceHealthStats.html#MEASUREMENT_LAUNCH_COUNT)
        pub const MEASUREMENT_LAUNCH_COUNT : i32 = 50002;

        /// public static final [MEASUREMENT_START_SERVICE_COUNT](https://developer.android.com/reference/android/os/health/ServiceHealthStats.html#MEASUREMENT_START_SERVICE_COUNT)
        pub const MEASUREMENT_START_SERVICE_COUNT : i32 = 50001;
    }
}
