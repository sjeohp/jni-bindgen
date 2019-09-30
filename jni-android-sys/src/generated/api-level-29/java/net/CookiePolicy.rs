// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-CookiePolicy"))]
__jni_bindgen! {
    /// public interface [CookiePolicy](https://developer.android.com/reference/java/net/CookiePolicy.html)
    ///
    /// Required feature: java-net-CookiePolicy
    public interface CookiePolicy ("java/net/CookiePolicy") extends crate::java::lang::Object {

        /// [shouldAccept](https://developer.android.com/reference/java/net/CookiePolicy.html#shouldAccept(java.net.URI,%20java.net.HttpCookie))
        ///
        /// Required features: "java-net-HttpCookie", "java-net-URI"
        #[cfg(any(feature = "all", all(feature = "java-net-HttpCookie", feature = "java-net-URI")))]
        pub fn shouldAccept<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URI>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::HttpCookie>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/CookiePolicy", java.flags == PUBLIC | ABSTRACT, .name == "shouldAccept", .descriptor == "(Ljava/net/URI;Ljava/net/HttpCookie;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/CookiePolicy\0", "shouldAccept\0", "(Ljava/net/URI;Ljava/net/HttpCookie;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [ACCEPT_ALL](https://developer.android.com/reference/java/net/CookiePolicy.html#ACCEPT_ALL)
        ///
        /// Required feature: java-net-CookiePolicy
        #[cfg(any(feature = "all", feature = "java-net-CookiePolicy"))]
        pub fn ACCEPT_ALL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::CookiePolicy>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/CookiePolicy\0", "ACCEPT_ALL\0", "Ljava/net/CookiePolicy;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ACCEPT_NONE](https://developer.android.com/reference/java/net/CookiePolicy.html#ACCEPT_NONE)
        ///
        /// Required feature: java-net-CookiePolicy
        #[cfg(any(feature = "all", feature = "java-net-CookiePolicy"))]
        pub fn ACCEPT_NONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::CookiePolicy>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/CookiePolicy\0", "ACCEPT_NONE\0", "Ljava/net/CookiePolicy;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ACCEPT_ORIGINAL_SERVER](https://developer.android.com/reference/java/net/CookiePolicy.html#ACCEPT_ORIGINAL_SERVER)
        ///
        /// Required feature: java-net-CookiePolicy
        #[cfg(any(feature = "all", feature = "java-net-CookiePolicy"))]
        pub fn ACCEPT_ORIGINAL_SERVER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::CookiePolicy>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/CookiePolicy\0", "ACCEPT_ORIGINAL_SERVER\0", "Ljava/net/CookiePolicy;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
