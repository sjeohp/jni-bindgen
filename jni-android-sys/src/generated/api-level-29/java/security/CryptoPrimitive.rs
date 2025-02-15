// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
__jni_bindgen! {
    /// public enum [CryptoPrimitive](https://developer.android.com/reference/java/security/CryptoPrimitive.html)
    ///
    /// Required feature: java-security-CryptoPrimitive
    public enum CryptoPrimitive ("java/security/CryptoPrimitive") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/security/CryptoPrimitive.html#values())
        ///
        /// Required features: "java-security-CryptoPrimitive"
        #[cfg(any(feature = "all", all(feature = "java-security-CryptoPrimitive")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::CryptoPrimitive, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/CryptoPrimitive", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/security/CryptoPrimitive;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/CryptoPrimitive\0", "values\0", "()[Ljava/security/CryptoPrimitive;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/security/CryptoPrimitive.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-CryptoPrimitive"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-CryptoPrimitive")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/CryptoPrimitive", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/security/CryptoPrimitive;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/CryptoPrimitive\0", "valueOf\0", "(Ljava/lang/String;)Ljava/security/CryptoPrimitive;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [CryptoPrimitive](https://developer.android.com/reference/java/security/CryptoPrimitive.html#CryptoPrimitive(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/CryptoPrimitive", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/CryptoPrimitive\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [MESSAGE_DIGEST](https://developer.android.com/reference/java/security/CryptoPrimitive.html#MESSAGE_DIGEST)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn MESSAGE_DIGEST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "MESSAGE_DIGEST\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SECURE_RANDOM](https://developer.android.com/reference/java/security/CryptoPrimitive.html#SECURE_RANDOM)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn SECURE_RANDOM<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "SECURE_RANDOM\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BLOCK_CIPHER](https://developer.android.com/reference/java/security/CryptoPrimitive.html#BLOCK_CIPHER)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn BLOCK_CIPHER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "BLOCK_CIPHER\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [STREAM_CIPHER](https://developer.android.com/reference/java/security/CryptoPrimitive.html#STREAM_CIPHER)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn STREAM_CIPHER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "STREAM_CIPHER\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MAC](https://developer.android.com/reference/java/security/CryptoPrimitive.html#MAC)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn MAC<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "MAC\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [KEY_WRAP](https://developer.android.com/reference/java/security/CryptoPrimitive.html#KEY_WRAP)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn KEY_WRAP<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "KEY_WRAP\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PUBLIC_KEY_ENCRYPTION](https://developer.android.com/reference/java/security/CryptoPrimitive.html#PUBLIC_KEY_ENCRYPTION)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn PUBLIC_KEY_ENCRYPTION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "PUBLIC_KEY_ENCRYPTION\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SIGNATURE](https://developer.android.com/reference/java/security/CryptoPrimitive.html#SIGNATURE)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn SIGNATURE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "SIGNATURE\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [KEY_ENCAPSULATION](https://developer.android.com/reference/java/security/CryptoPrimitive.html#KEY_ENCAPSULATION)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn KEY_ENCAPSULATION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "KEY_ENCAPSULATION\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [KEY_AGREEMENT](https://developer.android.com/reference/java/security/CryptoPrimitive.html#KEY_AGREEMENT)
        ///
        /// Required feature: java-security-CryptoPrimitive
        #[cfg(any(feature = "all", feature = "java-security-CryptoPrimitive"))]
        pub fn KEY_AGREEMENT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::CryptoPrimitive>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/CryptoPrimitive\0", "KEY_AGREEMENT\0", "Ljava/security/CryptoPrimitive;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::CryptoPrimitive, crate::java::lang::Throwable>>> { ... }
    }
}
