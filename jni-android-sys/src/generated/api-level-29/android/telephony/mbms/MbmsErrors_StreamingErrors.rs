// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-mbms-MbmsErrors_StreamingErrors"))]
__jni_bindgen! {
    /// public class [MbmsErrors.StreamingErrors](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.StreamingErrors.html)
    ///
    /// Required feature: android-telephony-mbms-MbmsErrors_StreamingErrors
    public class MbmsErrors_StreamingErrors ("android/telephony/mbms/MbmsErrors$StreamingErrors") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [StreamingErrors](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.StreamingErrors.html#StreamingErrors())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::mbms::MbmsErrors_StreamingErrors>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/mbms/MbmsErrors$StreamingErrors", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/mbms/MbmsErrors$StreamingErrors\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [ERROR_CONCURRENT_SERVICE_LIMIT_REACHED](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.StreamingErrors.html#ERROR_CONCURRENT_SERVICE_LIMIT_REACHED)
        pub const ERROR_CONCURRENT_SERVICE_LIMIT_REACHED : i32 = 301;

        /// public static final [ERROR_DUPLICATE_START_STREAM](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.StreamingErrors.html#ERROR_DUPLICATE_START_STREAM)
        pub const ERROR_DUPLICATE_START_STREAM : i32 = 303;

        /// public static final [ERROR_UNABLE_TO_START_SERVICE](https://developer.android.com/reference/android/telephony/mbms/MbmsErrors.StreamingErrors.html#ERROR_UNABLE_TO_START_SERVICE)
        pub const ERROR_UNABLE_TO_START_SERVICE : i32 = 302;
    }
}
