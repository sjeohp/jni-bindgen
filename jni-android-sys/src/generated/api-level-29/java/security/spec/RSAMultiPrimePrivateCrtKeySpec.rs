// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-spec-RSAMultiPrimePrivateCrtKeySpec"))]
__jni_bindgen! {
    /// public class [RSAMultiPrimePrivateCrtKeySpec](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html)
    ///
    /// Required feature: java-security-spec-RSAMultiPrimePrivateCrtKeySpec
    public class RSAMultiPrimePrivateCrtKeySpec ("java/security/spec/RSAMultiPrimePrivateCrtKeySpec") extends crate::java::security::spec::RSAPrivateKeySpec {

        /// [RSAMultiPrimePrivateCrtKeySpec](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#RSAMultiPrimePrivateCrtKeySpec(java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger,%20java.security.spec.RSAOtherPrimeInfo%5B%5D))
        ///
        /// Required features: "java-math-BigInteger", "java-security-spec-RSAOtherPrimeInfo"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger", feature = "java-security-spec-RSAOtherPrimeInfo")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg4: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg6: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg8: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::spec::RSAOtherPrimeInfo, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::spec::RSAMultiPrimePrivateCrtKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;[Ljava/security/spec/RSAOtherPrimeInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4.into()), __jni_bindgen::AsJValue::as_jvalue(&arg5.into()), __jni_bindgen::AsJValue::as_jvalue(&arg6.into()), __jni_bindgen::AsJValue::as_jvalue(&arg7.into()), __jni_bindgen::AsJValue::as_jvalue(&arg8.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "<init>\0", "(Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;[Ljava/security/spec/RSAOtherPrimeInfo;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPublicExponent](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getPublicExponent())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPublicExponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getPublicExponent", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getPublicExponent\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeP](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getPrimeP())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeP<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getPrimeP", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getPrimeP\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeQ](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getPrimeQ())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeQ<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getPrimeQ", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getPrimeQ\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeExponentP](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getPrimeExponentP())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeExponentP<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getPrimeExponentP", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getPrimeExponentP\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrimeExponentQ](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getPrimeExponentQ())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrimeExponentQ<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getPrimeExponentQ", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getPrimeExponentQ\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCrtCoefficient](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getCrtCoefficient())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getCrtCoefficient<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getCrtCoefficient", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getCrtCoefficient\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOtherPrimeInfo](https://developer.android.com/reference/java/security/spec/RSAMultiPrimePrivateCrtKeySpec.html#getOtherPrimeInfo())
        ///
        /// Required features: "java-security-spec-RSAOtherPrimeInfo"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-RSAOtherPrimeInfo")))]
        pub fn getOtherPrimeInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::spec::RSAOtherPrimeInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAMultiPrimePrivateCrtKeySpec", java.flags == PUBLIC, .name == "getOtherPrimeInfo", .descriptor == "()[Ljava/security/spec/RSAOtherPrimeInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAMultiPrimePrivateCrtKeySpec\0", "getOtherPrimeInfo\0", "()[Ljava/security/spec/RSAOtherPrimeInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
