// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-SSLSessionCache"))]
__jni_bindgen! {
    /// public final class [SSLSessionCache](https://developer.android.com/reference/android/net/SSLSessionCache.html)
    ///
    /// Required feature: android-net-SSLSessionCache
    public final class SSLSessionCache ("android/net/SSLSessionCache") extends crate::java::lang::Object {

        /// [SSLSessionCache](https://developer.android.com/reference/android/net/SSLSessionCache.html#SSLSessionCache(java.io.File))
        ///
        /// Required features: "java-io-File"
        #[cfg(any(feature = "all", all(feature = "java-io-File")))]
        pub fn new_File<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::File>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::SSLSessionCache>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLSessionCache", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/File;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLSessionCache\0", "<init>\0", "(Ljava/io/File;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SSLSessionCache](https://developer.android.com/reference/android/net/SSLSessionCache.html#SSLSessionCache(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn new_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::SSLSessionCache>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLSessionCache", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLSessionCache\0", "<init>\0", "(Landroid/content/Context;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
