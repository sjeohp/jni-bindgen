// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-AccessNetworkConstants_AccessNetworkType"))]
__jni_bindgen! {
    /// public final class [AccessNetworkConstants.AccessNetworkType](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html)
    ///
    /// Required feature: android-telephony-AccessNetworkConstants_AccessNetworkType
    public final class AccessNetworkConstants_AccessNetworkType ("android/telephony/AccessNetworkConstants$AccessNetworkType") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [AccessNetworkType](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#AccessNetworkType())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::AccessNetworkConstants_AccessNetworkType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/AccessNetworkConstants$AccessNetworkType", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/AccessNetworkConstants$AccessNetworkType\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [CDMA2000](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#CDMA2000)
        pub const CDMA2000 : i32 = 4;

        /// public static final [EUTRAN](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#EUTRAN)
        pub const EUTRAN : i32 = 3;

        /// public static final [GERAN](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#GERAN)
        pub const GERAN : i32 = 1;

        /// public static final [IWLAN](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#IWLAN)
        pub const IWLAN : i32 = 5;

        /// public static final [UNKNOWN](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#UNKNOWN)
        pub const UNKNOWN : i32 = 0;

        /// public static final [UTRAN](https://developer.android.com/reference/android/telephony/AccessNetworkConstants.AccessNetworkType.html#UTRAN)
        pub const UTRAN : i32 = 2;
    }
}
