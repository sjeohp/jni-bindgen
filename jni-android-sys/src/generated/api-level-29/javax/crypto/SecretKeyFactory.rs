// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-SecretKeyFactory"))]
__jni_bindgen! {
    /// public class [SecretKeyFactory](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html)
    ///
    /// Required feature: javax-crypto-SecretKeyFactory
    public class SecretKeyFactory ("javax/crypto/SecretKeyFactory") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SecretKeyFactory](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#SecretKeyFactory(javax.crypto.SecretKeyFactorySpi,%20java.security.Provider,%20java.lang.String))
        // ///
        // /// Required features: "java-lang-String", "java-security-Provider", "javax-crypto-SecretKeyFactorySpi"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "javax-crypto-SecretKeyFactorySpi")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::SecretKeyFactorySpi>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKeyFactory>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/SecretKeyFactory", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljavax/crypto/SecretKeyFactorySpi;Ljava/security/Provider;Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "<init>\0", "(Ljavax/crypto/SecretKeyFactorySpi;Ljava/security/Provider;Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getInstance(java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-crypto-SecretKeyFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-crypto-SecretKeyFactory")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKeyFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Ljavax/crypto/SecretKeyFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/crypto/SecretKeyFactory\0", "getInstance\0", "(Ljava/lang/String;)Ljavax/crypto/SecretKeyFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getInstance(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-crypto-SecretKeyFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-crypto-SecretKeyFactory")))]
        pub fn getInstance_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKeyFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/SecretKeyFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/crypto/SecretKeyFactory\0", "getInstance\0", "(Ljava/lang/String;Ljava/lang/String;)Ljavax/crypto/SecretKeyFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getInstance(java.lang.String,%20java.security.Provider))
        ///
        /// Required features: "java-lang-String", "java-security-Provider", "javax-crypto-SecretKeyFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "javax-crypto-SecretKeyFactory")))]
        pub fn getInstance_String_Provider<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKeyFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/SecretKeyFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/crypto/SecretKeyFactory\0", "getInstance\0", "(Ljava/lang/String;Ljava/security/Provider;)Ljavax/crypto/SecretKeyFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProvider](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getProvider())
        ///
        /// Required features: "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider")))]
        pub fn getProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Provider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | FINAL, .name == "getProvider", .descriptor == "()Ljava/security/Provider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "getProvider\0", "()Ljava/security/Provider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithm](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | FINAL, .name == "getAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "getAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [generateSecret](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#generateSecret(java.security.spec.KeySpec))
        ///
        /// Required features: "java-security-spec-KeySpec", "javax-crypto-SecretKey"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-KeySpec", feature = "javax-crypto-SecretKey")))]
        pub fn generateSecret<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::KeySpec>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | FINAL, .name == "generateSecret", .descriptor == "(Ljava/security/spec/KeySpec;)Ljavax/crypto/SecretKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "generateSecret\0", "(Ljava/security/spec/KeySpec;)Ljavax/crypto/SecretKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeySpec](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#getKeySpec(javax.crypto.SecretKey,%20java.lang.Class))
        ///
        /// Required features: "java-lang-Class", "java-security-spec-KeySpec", "javax-crypto-SecretKey"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-security-spec-KeySpec", feature = "javax-crypto-SecretKey")))]
        pub fn getKeySpec<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::SecretKey>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::KeySpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | FINAL, .name == "getKeySpec", .descriptor == "(Ljavax/crypto/SecretKey;Ljava/lang/Class;)Ljava/security/spec/KeySpec;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "getKeySpec\0", "(Ljavax/crypto/SecretKey;Ljava/lang/Class;)Ljava/security/spec/KeySpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [translateKey](https://developer.android.com/reference/javax/crypto/SecretKeyFactory.html#translateKey(javax.crypto.SecretKey))
        ///
        /// Required features: "javax-crypto-SecretKey"
        #[cfg(any(feature = "all", all(feature = "javax-crypto-SecretKey")))]
        pub fn translateKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::crypto::SecretKey>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::crypto::SecretKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/SecretKeyFactory", java.flags == PUBLIC | FINAL, .name == "translateKey", .descriptor == "(Ljavax/crypto/SecretKey;)Ljavax/crypto/SecretKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/SecretKeyFactory\0", "translateKey\0", "(Ljavax/crypto/SecretKey;)Ljavax/crypto/SecretKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
