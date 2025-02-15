// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-sip-SipSession_State"))]
__jni_bindgen! {
    /// public class [SipSession.State](https://developer.android.com/reference/android/net/sip/SipSession.State.html)
    ///
    /// Required feature: android-net-sip-SipSession_State
    public class SipSession_State ("android/net/sip/SipSession$State") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [State](https://developer.android.com/reference/android/net/sip/SipSession.State.html#State())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::sip::SipSession_State>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/sip/SipSession$State", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/sip/SipSession$State\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [toString](https://developer.android.com/reference/android/net/sip/SipSession.State.html#toString(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/sip/SipSession$State", java.flags == PUBLIC | STATIC, .name == "toString", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/sip/SipSession$State\0", "toString\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DEREGISTERING](https://developer.android.com/reference/android/net/sip/SipSession.State.html#DEREGISTERING)
        pub const DEREGISTERING : i32 = 2;

        /// public static final [INCOMING_CALL](https://developer.android.com/reference/android/net/sip/SipSession.State.html#INCOMING_CALL)
        pub const INCOMING_CALL : i32 = 3;

        /// public static final [INCOMING_CALL_ANSWERING](https://developer.android.com/reference/android/net/sip/SipSession.State.html#INCOMING_CALL_ANSWERING)
        pub const INCOMING_CALL_ANSWERING : i32 = 4;

        /// public static final [IN_CALL](https://developer.android.com/reference/android/net/sip/SipSession.State.html#IN_CALL)
        pub const IN_CALL : i32 = 8;

        /// public static final [NOT_DEFINED](https://developer.android.com/reference/android/net/sip/SipSession.State.html#NOT_DEFINED)
        pub const NOT_DEFINED : i32 = 101;

        /// public static final [OUTGOING_CALL](https://developer.android.com/reference/android/net/sip/SipSession.State.html#OUTGOING_CALL)
        pub const OUTGOING_CALL : i32 = 5;

        /// public static final [OUTGOING_CALL_CANCELING](https://developer.android.com/reference/android/net/sip/SipSession.State.html#OUTGOING_CALL_CANCELING)
        pub const OUTGOING_CALL_CANCELING : i32 = 7;

        /// public static final [OUTGOING_CALL_RING_BACK](https://developer.android.com/reference/android/net/sip/SipSession.State.html#OUTGOING_CALL_RING_BACK)
        pub const OUTGOING_CALL_RING_BACK : i32 = 6;

        /// public static final [PINGING](https://developer.android.com/reference/android/net/sip/SipSession.State.html#PINGING)
        pub const PINGING : i32 = 9;

        /// public static final [READY_TO_CALL](https://developer.android.com/reference/android/net/sip/SipSession.State.html#READY_TO_CALL)
        pub const READY_TO_CALL : i32 = 0;

        /// public static final [REGISTERING](https://developer.android.com/reference/android/net/sip/SipSession.State.html#REGISTERING)
        pub const REGISTERING : i32 = 1;
    }
}
