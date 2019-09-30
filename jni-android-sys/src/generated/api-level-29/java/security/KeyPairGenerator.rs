// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-KeyPairGenerator"))]
__jni_bindgen! {
    /// public class [KeyPairGenerator](https://developer.android.com/reference/java/security/KeyPairGenerator.html)
    ///
    /// Required feature: java-security-KeyPairGenerator
    public class KeyPairGenerator ("java/security/KeyPairGenerator") extends crate::java::security::KeyPairGeneratorSpi {

        // // Not emitting: Non-public method
        // /// [KeyPairGenerator](https://developer.android.com/reference/java/security/KeyPairGenerator.html#KeyPairGenerator(java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::KeyPairGenerator>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/KeyPairGenerator", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "<init>\0", "(Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getAlgorithm](https://developer.android.com/reference/java/security/KeyPairGenerator.html#getAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "getAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "getAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/KeyPairGenerator.html#getInstance(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-KeyPairGenerator"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-KeyPairGenerator")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPairGenerator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Ljava/security/KeyPairGenerator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/KeyPairGenerator\0", "getInstance\0", "(Ljava/lang/String;)Ljava/security/KeyPairGenerator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/KeyPairGenerator.html#getInstance(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-KeyPairGenerator"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-KeyPairGenerator")))]
        pub fn getInstance_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPairGenerator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyPairGenerator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/KeyPairGenerator\0", "getInstance\0", "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyPairGenerator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/KeyPairGenerator.html#getInstance(java.lang.String,%20java.security.Provider))
        ///
        /// Required features: "java-lang-String", "java-security-KeyPairGenerator", "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-KeyPairGenerator", feature = "java-security-Provider")))]
        pub fn getInstance_String_Provider<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPairGenerator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/KeyPairGenerator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/KeyPairGenerator\0", "getInstance\0", "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/KeyPairGenerator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProvider](https://developer.android.com/reference/java/security/KeyPairGenerator.html#getProvider())
        ///
        /// Required features: "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider")))]
        pub fn getProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Provider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC | FINAL, .name == "getProvider", .descriptor == "()Ljava/security/Provider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "getProvider\0", "()Ljava/security/Provider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initialize](https://developer.android.com/reference/java/security/KeyPairGenerator.html#initialize(int))
        pub fn initialize_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "initialize", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "initialize\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initialize](https://developer.android.com/reference/java/security/KeyPairGenerator.html#initialize(int,%20java.security.SecureRandom))
        ///
        /// Required features: "java-security-SecureRandom"
        #[cfg(any(feature = "all", all(feature = "java-security-SecureRandom")))]
        pub fn initialize_int_SecureRandom<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::SecureRandom>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "initialize", .descriptor == "(ILjava/security/SecureRandom;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "initialize\0", "(ILjava/security/SecureRandom;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initialize](https://developer.android.com/reference/java/security/KeyPairGenerator.html#initialize(java.security.spec.AlgorithmParameterSpec))
        ///
        /// Required features: "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-AlgorithmParameterSpec")))]
        pub fn initialize_AlgorithmParameterSpec<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "initialize", .descriptor == "(Ljava/security/spec/AlgorithmParameterSpec;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "initialize\0", "(Ljava/security/spec/AlgorithmParameterSpec;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initialize](https://developer.android.com/reference/java/security/KeyPairGenerator.html#initialize(java.security.spec.AlgorithmParameterSpec,%20java.security.SecureRandom))
        ///
        /// Required features: "java-security-SecureRandom", "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-SecureRandom", feature = "java-security-spec-AlgorithmParameterSpec")))]
        pub fn initialize_AlgorithmParameterSpec_SecureRandom<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::SecureRandom>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "initialize", .descriptor == "(Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "initialize\0", "(Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [genKeyPair](https://developer.android.com/reference/java/security/KeyPairGenerator.html#genKeyPair())
        ///
        /// Required features: "java-security-KeyPair"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyPair")))]
        pub fn genKeyPair<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPair>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC | FINAL, .name == "genKeyPair", .descriptor == "()Ljava/security/KeyPair;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "genKeyPair\0", "()Ljava/security/KeyPair;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [generateKeyPair](https://developer.android.com/reference/java/security/KeyPairGenerator.html#generateKeyPair())
        ///
        /// Required features: "java-security-KeyPair"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyPair")))]
        pub fn generateKeyPair<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPair>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyPairGenerator", java.flags == PUBLIC, .name == "generateKeyPair", .descriptor == "()Ljava/security/KeyPair;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyPairGenerator\0", "generateKeyPair\0", "()Ljava/security/KeyPair;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
