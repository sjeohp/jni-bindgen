// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-Proxy_Type"))]
__jni_bindgen! {
    /// public enum [Proxy.Type](https://developer.android.com/reference/java/net/Proxy.Type.html)
    ///
    /// Required feature: java-net-Proxy_Type
    public enum Proxy_Type ("java/net/Proxy$Type") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/net/Proxy.Type.html#values())
        ///
        /// Required features: "java-net-Proxy_Type"
        #[cfg(any(feature = "all", all(feature = "java-net-Proxy_Type")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::net::Proxy_Type, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy$Type", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/net/Proxy$Type;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/Proxy$Type\0", "values\0", "()[Ljava/net/Proxy$Type;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/net/Proxy.Type.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-net-Proxy_Type"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-Proxy_Type")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy$Type", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/net/Proxy$Type;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/Proxy$Type\0", "valueOf\0", "(Ljava/lang/String;)Ljava/net/Proxy$Type;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Type](https://developer.android.com/reference/java/net/Proxy.Type.html#Type(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/Proxy$Type", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy$Type\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [DIRECT](https://developer.android.com/reference/java/net/Proxy.Type.html#DIRECT)
        ///
        /// Required feature: java-net-Proxy_Type
        #[cfg(any(feature = "all", feature = "java-net-Proxy_Type"))]
        pub fn DIRECT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/Proxy$Type\0", "DIRECT\0", "Ljava/net/Proxy$Type;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HTTP](https://developer.android.com/reference/java/net/Proxy.Type.html#HTTP)
        ///
        /// Required feature: java-net-Proxy_Type
        #[cfg(any(feature = "all", feature = "java-net-Proxy_Type"))]
        pub fn HTTP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/Proxy$Type\0", "HTTP\0", "Ljava/net/Proxy$Type;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SOCKS](https://developer.android.com/reference/java/net/Proxy.Type.html#SOCKS)
        ///
        /// Required feature: java-net-Proxy_Type
        #[cfg(any(feature = "all", feature = "java-net-Proxy_Type"))]
        pub fn SOCKS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/Proxy$Type\0", "SOCKS\0", "Ljava/net/Proxy$Type;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::net::Proxy_Type, crate::java::lang::Throwable>>> { ... }
    }
}
