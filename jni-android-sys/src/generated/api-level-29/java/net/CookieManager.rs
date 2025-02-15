// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-CookieManager"))]
__jni_bindgen! {
    /// public class [CookieManager](https://developer.android.com/reference/java/net/CookieManager.html)
    ///
    /// Required feature: java-net-CookieManager
    public class CookieManager ("java/net/CookieManager") extends crate::java::net::CookieHandler {

        /// [CookieManager](https://developer.android.com/reference/java/net/CookieManager.html#CookieManager())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::CookieManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CookieManager](https://developer.android.com/reference/java/net/CookieManager.html#CookieManager(java.net.CookieStore,%20java.net.CookiePolicy))
        ///
        /// Required features: "java-net-CookiePolicy", "java-net-CookieStore"
        #[cfg(any(feature = "all", all(feature = "java-net-CookiePolicy", feature = "java-net-CookieStore")))]
        pub fn new_CookieStore_CookiePolicy<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::CookieStore>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::CookiePolicy>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::CookieManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/net/CookieStore;Ljava/net/CookiePolicy;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "<init>\0", "(Ljava/net/CookieStore;Ljava/net/CookiePolicy;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCookiePolicy](https://developer.android.com/reference/java/net/CookieManager.html#setCookiePolicy(java.net.CookiePolicy))
        ///
        /// Required features: "java-net-CookiePolicy"
        #[cfg(any(feature = "all", all(feature = "java-net-CookiePolicy")))]
        pub fn setCookiePolicy<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::CookiePolicy>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "setCookiePolicy", .descriptor == "(Ljava/net/CookiePolicy;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "setCookiePolicy\0", "(Ljava/net/CookiePolicy;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCookieStore](https://developer.android.com/reference/java/net/CookieManager.html#getCookieStore())
        ///
        /// Required features: "java-net-CookieStore"
        #[cfg(any(feature = "all", all(feature = "java-net-CookieStore")))]
        pub fn getCookieStore<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::CookieStore>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "getCookieStore", .descriptor == "()Ljava/net/CookieStore;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "getCookieStore\0", "()Ljava/net/CookieStore;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/net/CookieManager.html#get(java.net.URI,%20java.util.Map))
        ///
        /// Required features: "java-net-URI", "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-net-URI", feature = "java-util-Map")))]
        pub fn get<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URI>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Map>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "get", .descriptor == "(Ljava/net/URI;Ljava/util/Map;)Ljava/util/Map;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "get\0", "(Ljava/net/URI;Ljava/util/Map;)Ljava/util/Map;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/net/CookieManager.html#put(java.net.URI,%20java.util.Map))
        ///
        /// Required features: "java-net-URI", "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-net-URI", feature = "java-util-Map")))]
        pub fn put<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URI>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookieManager", java.flags == PUBLIC, .name == "put", .descriptor == "(Ljava/net/URI;Ljava/util/Map;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookieManager\0", "put\0", "(Ljava/net/URI;Ljava/util/Map;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
