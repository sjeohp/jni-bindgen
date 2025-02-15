// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-sip-SipRegistrationListener"))]
__jni_bindgen! {
    /// public interface [SipRegistrationListener](https://developer.android.com/reference/android/net/sip/SipRegistrationListener.html)
    ///
    /// Required feature: android-net-sip-SipRegistrationListener
    public interface SipRegistrationListener ("android/net/sip/SipRegistrationListener") extends crate::java::lang::Object {

        /// [onRegistering](https://developer.android.com/reference/android/net/sip/SipRegistrationListener.html#onRegistering(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onRegistering<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/sip/SipRegistrationListener", java.flags == PUBLIC | ABSTRACT, .name == "onRegistering", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/sip/SipRegistrationListener\0", "onRegistering\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRegistrationDone](https://developer.android.com/reference/android/net/sip/SipRegistrationListener.html#onRegistrationDone(java.lang.String,%20long))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onRegistrationDone<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/sip/SipRegistrationListener", java.flags == PUBLIC | ABSTRACT, .name == "onRegistrationDone", .descriptor == "(Ljava/lang/String;J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/sip/SipRegistrationListener\0", "onRegistrationDone\0", "(Ljava/lang/String;J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRegistrationFailed](https://developer.android.com/reference/android/net/sip/SipRegistrationListener.html#onRegistrationFailed(java.lang.String,%20int,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn onRegistrationFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/sip/SipRegistrationListener", java.flags == PUBLIC | ABSTRACT, .name == "onRegistrationFailed", .descriptor == "(Ljava/lang/String;ILjava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/sip/SipRegistrationListener\0", "onRegistrationFailed\0", "(Ljava/lang/String;ILjava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
