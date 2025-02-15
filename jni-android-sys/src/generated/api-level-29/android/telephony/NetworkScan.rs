// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telephony-NetworkScan"))]
__jni_bindgen! {
    /// public class [NetworkScan](https://developer.android.com/reference/android/telephony/NetworkScan.html)
    ///
    /// Required feature: android-telephony-NetworkScan
    public class NetworkScan ("android/telephony/NetworkScan") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [NetworkScan](https://developer.android.com/reference/android/telephony/NetworkScan.html#NetworkScan(int,%20int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telephony::NetworkScan>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telephony/NetworkScan", java.flags == (empty), .name == "<init>", .descriptor == "(II)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/NetworkScan\0", "<init>\0", "(II)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [stopScan](https://developer.android.com/reference/android/telephony/NetworkScan.html#stopScan())
        pub fn stopScan<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telephony/NetworkScan", java.flags == PUBLIC, .name == "stopScan", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telephony/NetworkScan\0", "stopScan\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ERROR_INTERRUPTED](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_INTERRUPTED)
        pub const ERROR_INTERRUPTED : i32 = 10002;

        /// public static final [ERROR_INVALID_SCAN](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_INVALID_SCAN)
        pub const ERROR_INVALID_SCAN : i32 = 2;

        /// public static final [ERROR_INVALID_SCANID](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_INVALID_SCANID)
        pub const ERROR_INVALID_SCANID : i32 = 10001;

        /// public static final [ERROR_MODEM_ERROR](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_MODEM_ERROR)
        pub const ERROR_MODEM_ERROR : i32 = 1;

        /// public static final [ERROR_MODEM_UNAVAILABLE](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_MODEM_UNAVAILABLE)
        pub const ERROR_MODEM_UNAVAILABLE : i32 = 3;

        /// public static final [ERROR_RADIO_INTERFACE_ERROR](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_RADIO_INTERFACE_ERROR)
        pub const ERROR_RADIO_INTERFACE_ERROR : i32 = 10000;

        /// public static final [ERROR_UNSUPPORTED](https://developer.android.com/reference/android/telephony/NetworkScan.html#ERROR_UNSUPPORTED)
        pub const ERROR_UNSUPPORTED : i32 = 4;

        /// public static final [SUCCESS](https://developer.android.com/reference/android/telephony/NetworkScan.html#SUCCESS)
        pub const SUCCESS : i32 = 0;
    }
}
