// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-interfaces-RSAMultiPrimePrivateCrtKey"))]
__jni_bindgen! {
    /// public interface [RSAMultiPrimePrivateCrtKey](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html)
    ///
    /// Required feature: java-security-interfaces-RSAMultiPrimePrivateCrtKey
    public interface RSAMultiPrimePrivateCrtKey ("java/security/interfaces/RSAMultiPrimePrivateCrtKey") extends crate::java::lang::Object, implements crate::java::security::interfaces::RSAPrivateKey {

        /// [getPublicExponent](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getPublicExponent())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPublicExponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getPublicExponent", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getPublicExponent\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeP](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getPrimeP())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeP<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getPrimeP", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getPrimeP\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeQ](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getPrimeQ())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeQ<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getPrimeQ", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getPrimeQ\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeExponentP](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getPrimeExponentP())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeExponentP<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getPrimeExponentP", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getPrimeExponentP\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeExponentQ](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getPrimeExponentQ())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeExponentQ<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getPrimeExponentQ", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getPrimeExponentQ\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCrtCoefficient](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getCrtCoefficient())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getCrtCoefficient<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getCrtCoefficient", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getCrtCoefficient\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOtherPrimeInfo](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#getOtherPrimeInfo())
        ///
        /// Required features: "java-security-spec-RSAOtherPrimeInfo"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-RSAOtherPrimeInfo")))]
        pub fn getOtherPrimeInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::spec::RSAOtherPrimeInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAMultiPrimePrivateCrtKey", java.flags == PUBLIC | ABSTRACT, .name == "getOtherPrimeInfo", .descriptor == "()[Ljava/security/spec/RSAOtherPrimeInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAMultiPrimePrivateCrtKey\0", "getOtherPrimeInfo\0", "()[Ljava/security/spec/RSAOtherPrimeInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [serialVersionUID](https://developer.android.com/reference/java/security/interfaces/RSAMultiPrimePrivateCrtKey.html#serialVersionUID)
        pub const serialVersionUID : i64 = 618058533534628008i64;
    }
}
