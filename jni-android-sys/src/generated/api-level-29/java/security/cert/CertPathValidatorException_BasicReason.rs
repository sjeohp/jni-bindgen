// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
__jni_bindgen! {
    /// public enum [CertPathValidatorException.BasicReason](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html)
    ///
    /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
    public enum CertPathValidatorException_BasicReason ("java/security/cert/CertPathValidatorException$BasicReason") extends crate::java::lang::Enum, implements crate::java::security::cert::CertPathValidatorException_Reason {

        /// [values](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#values())
        ///
        /// Required features: "java-security-cert-CertPathValidatorException_BasicReason"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPathValidatorException_BasicReason")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::CertPathValidatorException_BasicReason, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathValidatorException$BasicReason", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/security/cert/CertPathValidatorException$BasicReason;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathValidatorException$BasicReason\0", "values\0", "()[Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-cert-CertPathValidatorException_BasicReason"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-cert-CertPathValidatorException_BasicReason")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathValidatorException$BasicReason", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/security/cert/CertPathValidatorException$BasicReason;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathValidatorException$BasicReason\0", "valueOf\0", "(Ljava/lang/String;)Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [BasicReason](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#BasicReason(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/CertPathValidatorException$BasicReason", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathValidatorException$BasicReason\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [UNSPECIFIED](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#UNSPECIFIED)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn UNSPECIFIED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "UNSPECIFIED\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXPIRED](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#EXPIRED)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn EXPIRED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "EXPIRED\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NOT_YET_VALID](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#NOT_YET_VALID)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn NOT_YET_VALID<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "NOT_YET_VALID\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REVOKED](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#REVOKED)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn REVOKED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "REVOKED\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [UNDETERMINED_REVOCATION_STATUS](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#UNDETERMINED_REVOCATION_STATUS)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn UNDETERMINED_REVOCATION_STATUS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "UNDETERMINED_REVOCATION_STATUS\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVALID_SIGNATURE](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#INVALID_SIGNATURE)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn INVALID_SIGNATURE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "INVALID_SIGNATURE\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ALGORITHM_CONSTRAINED](https://developer.android.com/reference/java/security/cert/CertPathValidatorException.BasicReason.html#ALGORITHM_CONSTRAINED)
        ///
        /// Required feature: java-security-cert-CertPathValidatorException_BasicReason
        #[cfg(any(feature = "all", feature = "java-security-cert-CertPathValidatorException_BasicReason"))]
        pub fn ALGORITHM_CONSTRAINED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathValidatorException_BasicReason>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/security/cert/CertPathValidatorException$BasicReason\0", "ALGORITHM_CONSTRAINED\0", "Ljava/security/cert/CertPathValidatorException$BasicReason;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::CertPathValidatorException_BasicReason, crate::java::lang::Throwable>>> { ... }
    }
}
