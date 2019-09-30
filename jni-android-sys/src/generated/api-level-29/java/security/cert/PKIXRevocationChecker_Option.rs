// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-PKIXRevocationChecker_Option"))]
__jni_bindgen! {
    /// public enum [PKIXRevocationChecker.Option](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html)
    ///
    /// Required feature: java-security-cert-PKIXRevocationChecker_Option
    public enum PKIXRevocationChecker_Option ("java/security/cert/PKIXRevocationChecker$Option") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#values())
        ///
        /// Required features: "java-security-cert-PKIXRevocationChecker_Option"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-PKIXRevocationChecker_Option")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::PKIXRevocationChecker_Option, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXRevocationChecker$Option", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/security/cert/PKIXRevocationChecker$Option;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/PKIXRevocationChecker$Option\0", "values\0", "()[Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-cert-PKIXRevocationChecker_Option"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-cert-PKIXRevocationChecker_Option")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXRevocationChecker$Option", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/security/cert/PKIXRevocationChecker$Option;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/PKIXRevocationChecker$Option\0", "valueOf\0", "(Ljava/lang/String;)Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Option](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#Option(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/PKIXRevocationChecker$Option", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXRevocationChecker$Option\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [ONLY_END_ENTITY](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#ONLY_END_ENTITY)
        ///
        /// Required feature: java-security-cert-PKIXRevocationChecker_Option
        #[cfg(any(feature = "all", feature = "java-security-cert-PKIXRevocationChecker_Option"))]
        pub fn ONLY_END_ENTITY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/PKIXRevocationChecker$Option\0", "ONLY_END_ENTITY\0", "Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PREFER_CRLS](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#PREFER_CRLS)
        ///
        /// Required feature: java-security-cert-PKIXRevocationChecker_Option
        #[cfg(any(feature = "all", feature = "java-security-cert-PKIXRevocationChecker_Option"))]
        pub fn PREFER_CRLS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/PKIXRevocationChecker$Option\0", "PREFER_CRLS\0", "Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NO_FALLBACK](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#NO_FALLBACK)
        ///
        /// Required feature: java-security-cert-PKIXRevocationChecker_Option
        #[cfg(any(feature = "all", feature = "java-security-cert-PKIXRevocationChecker_Option"))]
        pub fn NO_FALLBACK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/PKIXRevocationChecker$Option\0", "NO_FALLBACK\0", "Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SOFT_FAIL](https://developer.android.com/reference/java/security/cert/PKIXRevocationChecker.Option.html#SOFT_FAIL)
        ///
        /// Required feature: java-security-cert-PKIXRevocationChecker_Option
        #[cfg(any(feature = "all", feature = "java-security-cert-PKIXRevocationChecker_Option"))]
        pub fn SOFT_FAIL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXRevocationChecker_Option>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/PKIXRevocationChecker$Option\0", "SOFT_FAIL\0", "Ljava/security/cert/PKIXRevocationChecker$Option;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::PKIXRevocationChecker_Option, crate::java::lang::Throwable>>> { ... }
    }
}
