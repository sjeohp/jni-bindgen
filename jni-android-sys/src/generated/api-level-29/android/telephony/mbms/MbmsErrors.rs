// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-mbms-MbmsErrors"))]
__jni_bindgen! {
    /// public class [MbmsErrors](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html)
    ///
    /// Required feature: android-telephony-mbms-MbmsErrors
    public class MbmsErrors ("android/telephony/mbms/MbmsErrors") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MbmsErrors](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#MbmsErrors())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::mbms::MbmsErrors>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/mbms/MbmsErrors", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsErrors\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [ERROR_MIDDLEWARE_LOST](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#ERROR_MIDDLEWARE_LOST)
        pub const ERROR_MIDDLEWARE_LOST : i32 = 3;

        /// public static final [ERROR_MIDDLEWARE_NOT_BOUND](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#ERROR_MIDDLEWARE_NOT_BOUND)
        pub const ERROR_MIDDLEWARE_NOT_BOUND : i32 = 2;

        /// public static final [ERROR_NO_UNIQUE_MIDDLEWARE](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#ERROR_NO_UNIQUE_MIDDLEWARE)
        pub const ERROR_NO_UNIQUE_MIDDLEWARE : i32 = 1;

        /// public static final [SUCCESS](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#SUCCESS)
        pub const SUCCESS : i32 = 0;

        /// public static final [UNKNOWN](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.html#UNKNOWN)
        pub const UNKNOWN : i32 = -1;
    }
}
