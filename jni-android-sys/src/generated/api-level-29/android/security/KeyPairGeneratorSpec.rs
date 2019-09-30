// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-security-KeyPairGeneratorSpec"))]
__jni_bindgen! {
    /// public final class [KeyPairGeneratorSpec](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html)
    ///
    /// Required feature: android-security-KeyPairGeneratorSpec
    #[deprecated] public final class KeyPairGeneratorSpec ("android/security/KeyPairGeneratorSpec") extends crate::java::lang::Object, implements crate::java::security::spec::AlgorithmParameterSpec {

        // // Not emitting: Non-public method
        // /// [KeyPairGeneratorSpec](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#KeyPairGeneratorSpec(android.content.Context,%20java.lang.String,%20java.lang.String,%20int,%20java.security.spec.AlgorithmParameterSpec,%20javax.security.auth.x500.X500Principal,%20java.math.BigInteger,%20java.util.Date,%20java.util.Date,%20int))
        // ///
        // /// Required features: "android-content-Context", "java-lang-String", "java-math-BigInteger", "java-security-spec-AlgorithmParameterSpec", "java-util-Date", "javax-security-auth-x500-X500Principal"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String", feature = "java-math-BigInteger", feature = "java-security-spec-AlgorithmParameterSpec", feature = "java-util-Date", feature = "javax-security-auth-x500-X500Principal")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: i32, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::security::auth::x500::X500Principal>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg8: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg9: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::security::KeyPairGeneratorSpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/security/KeyPairGeneratorSpec", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;ILjava/security/spec/AlgorithmParameterSpec;Ljavax/security/auth/x500/X500Principal;Ljava/math/BigInteger;Ljava/util/Date;Ljava/util/Date;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7.into()), __jni_bindgen::AsJValue::as_jvalue(&arg8.into()), __jni_bindgen::AsJValue::as_jvalue(&arg9)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "<init>\0", "(Landroid/content/Context;Ljava/lang/String;Ljava/lang/String;ILjava/security/spec/AlgorithmParameterSpec;Ljavax/security/auth/x500/X500Principal;Ljava/math/BigInteger;Ljava/util/Date;Ljava/util/Date;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getContext](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getContext())
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        #[deprecated] pub fn getContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Context>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getContext", .descriptor == "()Landroid/content/Context;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getContext\0", "()Landroid/content/Context;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeystoreAlias](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getKeystoreAlias())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn getKeystoreAlias<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getKeystoreAlias", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getKeystoreAlias\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyType](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getKeyType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn getKeyType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getKeyType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getKeyType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeySize](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getKeySize())
        #[deprecated] pub fn getKeySize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getKeySize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getKeySize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithmParameterSpec](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getAlgorithmParameterSpec())
        ///
        /// Required features: "java-security-spec-AlgorithmParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-AlgorithmParameterSpec")))]
        #[deprecated] pub fn getAlgorithmParameterSpec<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::AlgorithmParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getAlgorithmParameterSpec", .descriptor == "()Ljava/security/spec/AlgorithmParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getAlgorithmParameterSpec\0", "()Ljava/security/spec/AlgorithmParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubjectDN](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getSubjectDN())
        ///
        /// Required features: "javax-security-auth-x500-X500Principal"
        #[cfg(any(feature = "all", all(feature = "javax-security-auth-x500-X500Principal")))]
        #[deprecated] pub fn getSubjectDN<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::security::auth::x500::X500Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getSubjectDN", .descriptor == "()Ljavax/security/auth/x500/X500Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getSubjectDN\0", "()Ljavax/security/auth/x500/X500Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSerialNumber](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getSerialNumber())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        #[deprecated] pub fn getSerialNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getSerialNumber", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getSerialNumber\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStartDate](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getStartDate())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        #[deprecated] pub fn getStartDate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getStartDate", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getStartDate\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEndDate](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#getEndDate())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        #[deprecated] pub fn getEndDate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "getEndDate", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "getEndDate\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEncryptionRequired](https://developer.android.com/reference/android/security/KeyPairGeneratorSpec.html#isEncryptionRequired())
        #[deprecated] pub fn isEncryptionRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/KeyPairGeneratorSpec", java.flags == PUBLIC, .name == "isEncryptionRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/KeyPairGeneratorSpec\0", "isEncryptionRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
