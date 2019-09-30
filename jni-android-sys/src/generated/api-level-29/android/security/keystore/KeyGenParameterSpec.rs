// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-security-keystore-KeyGenParameterSpec"))]
__jni_bindgen! {
    /// public final class [KeyGenParameterSpec](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html)
    ///
    /// Required feature: android-security-keystore-KeyGenParameterSpec
    public final class KeyGenParameterSpec ("android/security/keystore/KeyGenParameterSpec") extends crate::java::lang::Object, implements crate::java::security::spec::AlgorithmParameterSpec {

        // // Not emitting: Non-public method
        // /// [KeyGenParameterSpec](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#KeyGenParameterSpec(java.lang.String,%20int,%20int,%20java.security.spec.AlgorithmParameterSpec,%20javax.security.auth.x500.X500Principal,%20java.math.BigInteger,%20java.util.Date,%20java.util.Date,%20java.util.Date,%20java.util.Date,%20java.util.Date,%20int,%20java.lang.String%5B%5D,%20java.lang.String%5B%5D,%20java.lang.String%5B%5D,%20java.lang.String%5B%5D,%20boolean,%20boolean,%20int,%20boolean,%20byte%5B%5D,%20boolean,%20boolean,%20boolean,%20boolean,%20boolean,%20boolean))
        // ///
        // /// Required features: "java-lang-String", "java-math-BigInteger", "java-security-spec-AlgorithmParameterSpec", "java-util-Date", "javax-security-auth-x500-X500Principal"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-math-BigInteger", feature = "java-security-spec-AlgorithmParameterSpec", feature = "java-util-Date", feature = "javax-security-auth-x500-X500Principal")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::security::auth::x500::X500Principal>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg8: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg9: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg10: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg11: i32, arg12: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg13: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg14: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg15: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg16: bool, arg17: bool, arg18: i32, arg19: bool, arg20: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg21: bool, arg22: bool, arg23: bool, arg24: bool, arg25: bool, arg26: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::security::keystore::KeyGenParameterSpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/lang/String;IILjava/security/spec/AlgorithmParameterSpec;Ljavax/security/auth/x500/X500Principal;Ljava/math/BigInteger;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;I[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;ZZIZ[BZZZZZZ)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7.into()), __jni_bindgen::AsJValue::as_jvalue(&arg8.into()), __jni_bindgen::AsJValue::as_jvalue(&arg9.into()), __jni_bindgen::AsJValue::as_jvalue(&arg10.into()), __jni_bindgen::AsJValue::as_jvalue(&arg11), __jni_bindgen::AsJValue::as_jvalue(&arg12.into()), __jni_bindgen::AsJValue::as_jvalue(&arg13.into()), __jni_bindgen::AsJValue::as_jvalue(&arg14.into()), __jni_bindgen::AsJValue::as_jvalue(&arg15.into()), __jni_bindgen::AsJValue::as_jvalue(&arg16), __jni_bindgen::AsJValue::as_jvalue(&arg17), __jni_bindgen::AsJValue::as_jvalue(&arg18), __jni_bindgen::AsJValue::as_jvalue(&arg19), __jni_bindgen::AsJValue::as_jvalue(&arg20.into()), __jni_bindgen::AsJValue::as_jvalue(&arg21), __jni_bindgen::AsJValue::as_jvalue(&arg22), __jni_bindgen::AsJValue::as_jvalue(&arg23), __jni_bindgen::AsJValue::as_jvalue(&arg24), __jni_bindgen::AsJValue::as_jvalue(&arg25), __jni_bindgen::AsJValue::as_jvalue(&arg26)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "<init>\0", "(Ljava/lang/String;IILjava/security/spec/AlgorithmParameterSpec;Ljavax/security/auth/x500/X500Principal;Ljava/math/BigInteger;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;Ljava/util/Date;I[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;ZZIZ[BZZZZZZ)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getKeystoreAlias](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getKeystoreAlias())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getKeystoreAlias<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getKeystoreAlias", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getKeystoreAlias\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeySize](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getKeySize())
        pub fn getKeySize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getKeySize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getKeySize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithmParameterSpec](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getAlgorithmParameterSpec())
        ///
        /// Required features: "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-AlgorithmParameterSpec")))]
        pub fn getAlgorithmParameterSpec<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::AlgorithmParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getAlgorithmParameterSpec", .descriptor == "()Ljava/security/spec/AlgorithmParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getAlgorithmParameterSpec\0", "()Ljava/security/spec/AlgorithmParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificateSubject](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getCertificateSubject())
        ///
        /// Required features: "javax-security-auth-x500-X500Principal"
        #[cfg(any(feature = "all", all(feature = "javax-security-auth-x500-X500Principal")))]
        pub fn getCertificateSubject<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::security::auth::x500::X500Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getCertificateSubject", .descriptor == "()Ljavax/security/auth/x500/X500Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getCertificateSubject\0", "()Ljavax/security/auth/x500/X500Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificateSerialNumber](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getCertificateSerialNumber())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getCertificateSerialNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getCertificateSerialNumber", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getCertificateSerialNumber\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificateNotBefore](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getCertificateNotBefore())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getCertificateNotBefore<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getCertificateNotBefore", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getCertificateNotBefore\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertificateNotAfter](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getCertificateNotAfter())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getCertificateNotAfter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getCertificateNotAfter", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getCertificateNotAfter\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyValidityStart](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getKeyValidityStart())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getKeyValidityStart<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getKeyValidityStart", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getKeyValidityStart\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyValidityForConsumptionEnd](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getKeyValidityForConsumptionEnd())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getKeyValidityForConsumptionEnd<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getKeyValidityForConsumptionEnd", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getKeyValidityForConsumptionEnd\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyValidityForOriginationEnd](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getKeyValidityForOriginationEnd())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getKeyValidityForOriginationEnd<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getKeyValidityForOriginationEnd", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getKeyValidityForOriginationEnd\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPurposes](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getPurposes())
        pub fn getPurposes<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getPurposes", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getPurposes\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDigests](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getDigests())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDigests<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getDigests", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getDigests\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDigestsSpecified](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isDigestsSpecified())
        pub fn isDigestsSpecified<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isDigestsSpecified", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isDigestsSpecified\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncryptionPaddings](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getEncryptionPaddings())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEncryptionPaddings<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getEncryptionPaddings", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getEncryptionPaddings\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSignaturePaddings](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getSignaturePaddings())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSignaturePaddings<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getSignaturePaddings", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getSignaturePaddings\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBlockModes](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getBlockModes())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getBlockModes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getBlockModes", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getBlockModes\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRandomizedEncryptionRequired](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isRandomizedEncryptionRequired())
        pub fn isRandomizedEncryptionRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isRandomizedEncryptionRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isRandomizedEncryptionRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isUserAuthenticationRequired](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isUserAuthenticationRequired())
        pub fn isUserAuthenticationRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isUserAuthenticationRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isUserAuthenticationRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isUserConfirmationRequired](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isUserConfirmationRequired())
        pub fn isUserConfirmationRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isUserConfirmationRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isUserConfirmationRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUserAuthenticationValidityDurationSeconds](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getUserAuthenticationValidityDurationSeconds())
        pub fn getUserAuthenticationValidityDurationSeconds<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getUserAuthenticationValidityDurationSeconds", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getUserAuthenticationValidityDurationSeconds\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isUserPresenceRequired](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isUserPresenceRequired())
        pub fn isUserPresenceRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isUserPresenceRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isUserPresenceRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttestationChallenge](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#getAttestationChallenge())
        pub fn getAttestationChallenge<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "getAttestationChallenge", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "getAttestationChallenge\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isUserAuthenticationValidWhileOnBody](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isUserAuthenticationValidWhileOnBody())
        pub fn isUserAuthenticationValidWhileOnBody<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isUserAuthenticationValidWhileOnBody", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isUserAuthenticationValidWhileOnBody\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isInvalidatedByBiometricEnrollment](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isInvalidatedByBiometricEnrollment())
        pub fn isInvalidatedByBiometricEnrollment<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isInvalidatedByBiometricEnrollment", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isInvalidatedByBiometricEnrollment\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isStrongBoxBacked](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isStrongBoxBacked())
        pub fn isStrongBoxBacked<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isStrongBoxBacked", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isStrongBoxBacked\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isUnlockedDeviceRequired](https://developer.android.com/reference/android/security/keystore/KeyGenParameterSpec.html#isUnlockedDeviceRequired())
        pub fn isUnlockedDeviceRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/keystore/KeyGenParameterSpec", java.flags == PUBLIC, .name == "isUnlockedDeviceRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/keystore/KeyGenParameterSpec\0", "isUnlockedDeviceRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
