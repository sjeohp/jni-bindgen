// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
__jni_bindgen! {
    /// public enum [SSLEngineResult.HandshakeStatus](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html)
    ///
    /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
    public enum SSLEngineResult_HandshakeStatus ("javax/net/ssl/SSLEngineResult$HandshakeStatus") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#values())
        ///
        /// Required features: "javax-net-ssl-SSLEngineResult_HandshakeStatus"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::javax::net::ssl::SSLEngineResult_HandshakeStatus, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult$HandshakeStatus", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "values\0", "()[Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-net-ssl-SSLEngineResult_HandshakeStatus"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult$HandshakeStatus", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "valueOf\0", "(Ljava/lang/String;)Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [HandshakeStatus](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#HandshakeStatus(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLEngineResult$HandshakeStatus", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [NOT_HANDSHAKING](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#NOT_HANDSHAKING)
        ///
        /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
        #[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
        pub fn NOT_HANDSHAKING<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "NOT_HANDSHAKING\0", "Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FINISHED](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#FINISHED)
        ///
        /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
        #[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
        pub fn FINISHED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "FINISHED\0", "Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEED_TASK](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#NEED_TASK)
        ///
        /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
        #[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
        pub fn NEED_TASK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "NEED_TASK\0", "Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEED_WRAP](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#NEED_WRAP)
        ///
        /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
        #[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
        pub fn NEED_WRAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "NEED_WRAP\0", "Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NEED_UNWRAP](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.HandshakeStatus.html#NEED_UNWRAP)
        ///
        /// Required feature: javax-net-ssl-SSLEngineResult_HandshakeStatus
        #[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus"))]
        pub fn NEED_UNWRAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("javax/net/ssl/SSLEngineResult$HandshakeStatus\0", "NEED_UNWRAP\0", "Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::javax::net::ssl::SSLEngineResult_HandshakeStatus, crate::java::lang::Throwable>>> { ... }
    }
}
