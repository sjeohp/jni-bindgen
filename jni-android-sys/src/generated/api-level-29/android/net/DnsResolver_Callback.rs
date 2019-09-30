// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-DnsResolver_Callback"))]
__jni_bindgen! {
    /// public interface [DnsResolver.Callback](https://developer.android.com/reference/android/net/DnsResolver.Callback.html)
    ///
    /// Required feature: android-net-DnsResolver_Callback
    public interface DnsResolver_Callback ("android/net/DnsResolver$Callback") extends crate::java::lang::Object {

        /// [onAnswer](https://developer.android.com/reference/android/net/DnsResolver.Callback.html#onAnswer(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn onAnswer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/DnsResolver$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onAnswer", .descriptor == "(Ljava/lang/Object;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/DnsResolver$Callback\0", "onAnswer\0", "(Ljava/lang/Object;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onError](https://developer.android.com/reference/android/net/DnsResolver.Callback.html#onError(android.net.DnsResolver.DnsException))
        ///
        /// Required features: "android-net-DnsResolver_DnsException"
        #[cfg(any(feature = "all", all(feature = "android-net-DnsResolver_DnsException")))]
        pub fn onError<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::DnsResolver_DnsException>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/DnsResolver$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onError", .descriptor == "(Landroid/net/DnsResolver$DnsException;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/DnsResolver$Callback\0", "onError\0", "(Landroid/net/DnsResolver$DnsException;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
