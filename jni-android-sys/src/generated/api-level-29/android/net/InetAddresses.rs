// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-InetAddresses"))]
__jni_bindgen! {
    /// public class [InetAddresses](https://developer.android.com/reference/android/net/InetAddresses.html)
    ///
    /// Required feature: android-net-InetAddresses
    public class InetAddresses ("android/net/InetAddresses") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [InetAddresses](https://developer.android.com/reference/android/net/InetAddresses.html#InetAddresses())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::InetAddresses>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/InetAddresses", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/InetAddresses\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isNumericAddress](https://developer.android.com/reference/android/net/InetAddresses.html#isNumericAddress(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn isNumericAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/InetAddresses", java.flags == PUBLIC | STATIC, .name == "isNumericAddress", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/InetAddresses\0", "isNumericAddress\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseNumericAddress](https://developer.android.com/reference/android/net/InetAddresses.html#parseNumericAddress(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-InetAddress")))]
        pub fn parseNumericAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/InetAddresses", java.flags == PUBLIC | STATIC, .name == "parseNumericAddress", .descriptor == "(Ljava/lang/String;)Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/InetAddresses\0", "parseNumericAddress\0", "(Ljava/lang/String;)Ljava/net/InetAddress;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
