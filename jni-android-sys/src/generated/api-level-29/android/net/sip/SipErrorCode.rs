// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-sip-SipErrorCode"))]
__jni_bindgen! {
    /// public class [SipErrorCode](https://developer.android.com/reference/android/net/sip/SipErrorCode.html)
    ///
    /// Required feature: android-net-sip-SipErrorCode
    public class SipErrorCode ("android/net/sip/SipErrorCode") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SipErrorCode](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#SipErrorCode())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::sip::SipErrorCode>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/sip/SipErrorCode", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/sip/SipErrorCode\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [toString](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#toString(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/sip/SipErrorCode", java.flags == PUBLIC | STATIC, .name == "toString", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/sip/SipErrorCode\0", "toString\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CLIENT_ERROR](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#CLIENT_ERROR)
        pub const CLIENT_ERROR : i32 = -4;

        /// public static final [CROSS_DOMAIN_AUTHENTICATION](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#CROSS_DOMAIN_AUTHENTICATION)
        pub const CROSS_DOMAIN_AUTHENTICATION : i32 = -11;

        /// public static final [DATA_CONNECTION_LOST](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#DATA_CONNECTION_LOST)
        pub const DATA_CONNECTION_LOST : i32 = -10;

        /// public static final [INVALID_CREDENTIALS](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#INVALID_CREDENTIALS)
        pub const INVALID_CREDENTIALS : i32 = -8;

        /// public static final [INVALID_REMOTE_URI](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#INVALID_REMOTE_URI)
        pub const INVALID_REMOTE_URI : i32 = -6;

        /// public static final [IN_PROGRESS](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#IN_PROGRESS)
        pub const IN_PROGRESS : i32 = -9;

        /// public static final [NO_ERROR](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#NO_ERROR)
        pub const NO_ERROR : i32 = 0;

        /// public static final [PEER_NOT_REACHABLE](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#PEER_NOT_REACHABLE)
        pub const PEER_NOT_REACHABLE : i32 = -7;

        /// public static final [SERVER_ERROR](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#SERVER_ERROR)
        pub const SERVER_ERROR : i32 = -2;

        /// public static final [SERVER_UNREACHABLE](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#SERVER_UNREACHABLE)
        pub const SERVER_UNREACHABLE : i32 = -12;

        /// public static final [SOCKET_ERROR](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#SOCKET_ERROR)
        pub const SOCKET_ERROR : i32 = -1;

        /// public static final [TIME_OUT](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#TIME_OUT)
        pub const TIME_OUT : i32 = -5;

        /// public static final [TRANSACTION_TERMINTED](https://developer.android.com/reference/android/net/sip/SipErrorCode.html#TRANSACTION_TERMINTED)
        pub const TRANSACTION_TERMINTED : i32 = -3;
    }
}
