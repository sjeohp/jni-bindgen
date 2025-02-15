// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-WebResourceRequest"))]
__jni_bindgen! {
    /// public interface [WebResourceRequest](https://developer.android.com/reference/android/webkit/WebResourceRequest.html)
    ///
    /// Required feature: android-webkit-WebResourceRequest
    public interface WebResourceRequest ("android/webkit/WebResourceRequest") extends crate::java::lang::Object {

        /// [getUrl](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#getUrl())
        ///
        /// Required features: "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri")))]
        pub fn getUrl<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::net::Uri>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "getUrl", .descriptor == "()Landroid/net/Uri;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "getUrl\0", "()Landroid/net/Uri;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isForMainFrame](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#isForMainFrame())
        pub fn isForMainFrame<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "isForMainFrame", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "isForMainFrame\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRedirect](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#isRedirect())
        pub fn isRedirect<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "isRedirect", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "isRedirect\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasGesture](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#hasGesture())
        pub fn hasGesture<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "hasGesture", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "hasGesture\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMethod](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#getMethod())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getMethod<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "getMethod", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "getMethod\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRequestHeaders](https://developer.android.com/reference/android/webkit/WebResourceRequest.html#getRequestHeaders())
        ///
        /// Required features: "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-util-Map")))]
        pub fn getRequestHeaders<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Map>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/WebResourceRequest", java.flags == PUBLIC | ABSTRACT, .name == "getRequestHeaders", .descriptor == "()Ljava/util/Map;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/WebResourceRequest\0", "getRequestHeaders\0", "()Ljava/util/Map;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
