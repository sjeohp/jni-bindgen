// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-Signature"))]
__jni_bindgen! {
    /// public class [Signature](https://developer.android.com/reference/java/security/Signature.html)
    ///
    /// Required feature: java-security-Signature
    public class Signature ("java/security/Signature") extends crate::java::security::SignatureSpi {

        // // Not emitting: Non-public method
        // /// [Signature](https://developer.android.com/reference/java/security/Signature.html#Signature(java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::Signature>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/Signature", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "<init>\0", "(Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/java/security/Signature.html#getInstance(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-Signature"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Signature")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Signature>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Ljava/security/Signature;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/Signature\0", "getInstance\0", "(Ljava/lang/String;)Ljava/security/Signature;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/Signature.html#getInstance(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-security-Signature"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Signature")))]
        pub fn getInstance_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Signature>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Signature;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/Signature\0", "getInstance\0", "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Signature;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/java/security/Signature.html#getInstance(java.lang.String,%20java.security.Provider))
        ///
        /// Required features: "java-lang-String", "java-security-Provider", "java-security-Signature"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "java-security-Signature")))]
        pub fn getInstance_String_Provider<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Signature>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/Signature;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/security/Signature\0", "getInstance\0", "(Ljava/lang/String;Ljava/security/Provider;)Ljava/security/Signature;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProvider](https://developer.android.com/reference/java/security/Signature.html#getProvider())
        ///
        /// Required features: "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider")))]
        pub fn getProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Provider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "getProvider", .descriptor == "()Ljava/security/Provider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "getProvider\0", "()Ljava/security/Provider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initVerify](https://developer.android.com/reference/java/security/Signature.html#initVerify(java.security.PublicKey))
        ///
        /// Required features: "java-security-PublicKey"
        #[cfg(any(feature = "all", all(feature = "java-security-PublicKey")))]
        pub fn initVerify_PublicKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PublicKey>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "initVerify", .descriptor == "(Ljava/security/PublicKey;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "initVerify\0", "(Ljava/security/PublicKey;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initVerify](https://developer.android.com/reference/java/security/Signature.html#initVerify(java.security.cert.Certificate))
        ///
        /// Required features: "java-security-cert-Certificate"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-Certificate")))]
        pub fn initVerify_Certificate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::Certificate>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "initVerify", .descriptor == "(Ljava/security/cert/Certificate;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "initVerify\0", "(Ljava/security/cert/Certificate;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initSign](https://developer.android.com/reference/java/security/Signature.html#initSign(java.security.PrivateKey))
        ///
        /// Required features: "java-security-PrivateKey"
        #[cfg(any(feature = "all", all(feature = "java-security-PrivateKey")))]
        pub fn initSign_PrivateKey<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PrivateKey>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "initSign", .descriptor == "(Ljava/security/PrivateKey;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "initSign\0", "(Ljava/security/PrivateKey;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [initSign](https://developer.android.com/reference/java/security/Signature.html#initSign(java.security.PrivateKey,%20java.security.SecureRandom))
        ///
        /// Required features: "java-security-PrivateKey", "java-security-SecureRandom"
        #[cfg(any(feature = "all", all(feature = "java-security-PrivateKey", feature = "java-security-SecureRandom")))]
        pub fn initSign_PrivateKey_SecureRandom<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PrivateKey>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::SecureRandom>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "initSign", .descriptor == "(Ljava/security/PrivateKey;Ljava/security/SecureRandom;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "initSign\0", "(Ljava/security/PrivateKey;Ljava/security/SecureRandom;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sign](https://developer.android.com/reference/java/security/Signature.html#sign())
        pub fn sign<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "sign", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "sign\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sign](https://developer.android.com/reference/java/security/Signature.html#sign(byte%5B%5D,%20int,%20int))
        pub fn sign_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "sign", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "sign\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [verify](https://developer.android.com/reference/java/security/Signature.html#verify(byte%5B%5D))
        pub fn verify_byte_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "verify", .descriptor == "([B)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "verify\0", "([B)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [verify](https://developer.android.com/reference/java/security/Signature.html#verify(byte%5B%5D,%20int,%20int))
        pub fn verify_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "verify", .descriptor == "([BII)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "verify\0", "([BII)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [update](https://developer.android.com/reference/java/security/Signature.html#update(byte))
        pub fn update_byte<'env>(&'env self, arg0: i8) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "update", .descriptor == "(B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "update\0", "(B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [update](https://developer.android.com/reference/java/security/Signature.html#update(byte%5B%5D))
        pub fn update_byte_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "update", .descriptor == "([B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "update\0", "([B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [update](https://developer.android.com/reference/java/security/Signature.html#update(byte%5B%5D,%20int,%20int))
        pub fn update_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "update", .descriptor == "([BII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "update\0", "([BII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [update](https://developer.android.com/reference/java/security/Signature.html#update(java.nio.ByteBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn update_ByteBuffer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "update", .descriptor == "(Ljava/nio/ByteBuffer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "update\0", "(Ljava/nio/ByteBuffer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithm](https://developer.android.com/reference/java/security/Signature.html#getAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "getAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "getAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/Signature.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setParameter](https://developer.android.com/reference/java/security/Signature.html#setParameter(java.lang.String,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        #[deprecated] pub fn setParameter_String_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "setParameter", .descriptor == "(Ljava/lang/String;Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "setParameter\0", "(Ljava/lang/String;Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setParameter](https://developer.android.com/reference/java/security/Signature.html#setParameter(java.security.spec.AlgorithmParameterSpec))
        ///
        /// Required features: "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-AlgorithmParameterSpec")))]
        pub fn setParameter_AlgorithmParameterSpec<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "setParameter", .descriptor == "(Ljava/security/spec/AlgorithmParameterSpec;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "setParameter\0", "(Ljava/security/spec/AlgorithmParameterSpec;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParameters](https://developer.android.com/reference/java/security/Signature.html#getParameters())
        ///
        /// Required features: "java-security-AlgorithmParameters"
        #[cfg(any(feature = "all", all(feature = "java-security-AlgorithmParameters")))]
        pub fn getParameters<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::AlgorithmParameters>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "getParameters", .descriptor == "()Ljava/security/AlgorithmParameters;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "getParameters\0", "()Ljava/security/AlgorithmParameters;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParameter](https://developer.android.com/reference/java/security/Signature.html#getParameter(java.lang.String))
        ///
        /// Required features: "java-lang-Object", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String")))]
        #[deprecated] pub fn getParameter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC | FINAL, .name == "getParameter", .descriptor == "(Ljava/lang/String;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "getParameter\0", "(Ljava/lang/String;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/security/Signature.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Signature", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Signature\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// protected static final [SIGN](https://developer.android.com/reference/java/security/Signature.html#SIGN)
        // pub const SIGN : i32 = 2;

        // // Not emitting: Non-public field
        // /// protected static final [UNINITIALIZED](https://developer.android.com/reference/java/security/Signature.html#UNINITIALIZED)
        // pub const UNINITIALIZED : i32 = 0;

        // // Not emitting: Non-public field
        // /// protected static final [VERIFY](https://developer.android.com/reference/java/security/Signature.html#VERIFY)
        // pub const VERIFY : i32 = 3;

        // // Not emitting: Non-public field
        // /// **get** protected [state](https://developer.android.com/reference/java/security/Signature.html#state)
        // pub fn state<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/security/Signature\0", "state\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [state](https://developer.android.com/reference/java/security/Signature.html#state)
        // pub fn set_state<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/security/Signature\0", "state\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }
    }
}
