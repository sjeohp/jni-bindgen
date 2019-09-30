// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-CertPathBuilder"))]
__jni_bindgen! {
    /// public class [CertPathBuilder](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html)
    ///
    /// Required feature: java-security-cert-CertPathBuilder
    public class CertPathBuilder ("java/security/cert/CertPathBuilder") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [CertPathBuilder](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#CertPathBuilder(java.security.cert.CertPathBuilderSpi,%20java.security.Provider,%20java.lang.String))
        // ///
        // /// Required features: "java-lang-String", "java-security-Provider", "java-security-cert-CertPathBuilderSpi"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "java-security-cert-CertPathBuilderSpi")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertPathBuilderSpi>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathBuilder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/CertPathBuilder", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/security/cert/CertPathBuilderSpi;Ljava/security/Provider;Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathBuilder\0", "<init>\0", "(Ljava/security/cert/CertPathBuilderSpi;Ljava/security/Provider;Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getInstance(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-cert-CertPathBuilder"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-cert-CertPathBuilder")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathBuilder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathBuilder\0", "getInstance\0", "(Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getInstance(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-cert-CertPathBuilder"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-cert-CertPathBuilder")))]
        pub fn getInstance_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathBuilder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathBuilder\0", "getInstance\0", "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/cert/CertPathBuilder;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getInstance(java.lang.String,%20java.security.Provider))
        ///
        /// Required features: "java-lang-String", "java-security-Provider", "java-security-cert-CertPathBuilder"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "java-security-cert-CertPathBuilder")))]
        pub fn getInstance_String_Provider<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathBuilder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/cert/CertPathBuilder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathBuilder\0", "getInstance\0", "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/cert/CertPathBuilder;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProvider](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getProvider())
        ///
        /// Required features: "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider")))]
        pub fn getProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Provider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | FINAL, .name == "getProvider", .descriptor == "()Ljava/security/Provider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathBuilder\0", "getProvider\0", "()Ljava/security/Provider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithm](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | FINAL, .name == "getAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathBuilder\0", "getAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#build(java.security.cert.CertPathParameters))
        ///
        /// Required features: "java-security-cert-CertPathBuilderResult", "java-security-cert-CertPathParameters"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPathBuilderResult", feature = "java-security-cert-CertPathParameters")))]
        pub fn build<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertPathParameters>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathBuilderResult>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | FINAL, .name == "build", .descriptor == "(Ljava/security/cert/CertPathParameters;)Ljava/security/cert/CertPathBuilderResult;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathBuilder\0", "build\0", "(Ljava/security/cert/CertPathParameters;)Ljava/security/cert/CertPathBuilderResult;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultType](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getDefaultType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDefaultType<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | STATIC | FINAL, .name == "getDefaultType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/cert/CertPathBuilder\0", "getDefaultType\0", "()Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRevocationChecker](https://developer.android.com/reference/java/security/cert/CertPathBuilder.html#getRevocationChecker())
        ///
        /// Required features: "java-security-cert-CertPathChecker"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPathChecker")))]
        pub fn getRevocationChecker<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPathChecker>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/CertPathBuilder", java.flags == PUBLIC | FINAL, .name == "getRevocationChecker", .descriptor == "()Ljava/security/cert/CertPathChecker;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/CertPathBuilder\0", "getRevocationChecker\0", "()Ljava/security/cert/CertPathChecker;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
