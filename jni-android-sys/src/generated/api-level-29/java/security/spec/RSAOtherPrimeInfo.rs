// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-spec-RSAOtherPrimeInfo"))]
__jni_bindgen! {
    /// public class [RSAOtherPrimeInfo](https://developer.android.com/reference/java/security/spec/RSAOtherPrimeInfo.html)
    ///
    /// Required feature: java-security-spec-RSAOtherPrimeInfo
    public class RSAOtherPrimeInfo ("java/security/spec/RSAOtherPrimeInfo") extends crate::java::lang::Object {

        /// [RSAOtherPrimeInfo](https://developer.android.com/reference/java/security/spec/RSAOtherPrimeInfo.html#RSAOtherPrimeInfo(java.math.BigInteger,%20java.math.BigInteger,%20java.math.BigInteger))
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::spec::RSAOtherPrimeInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAOtherPrimeInfo", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAOtherPrimeInfo\0", "<init>\0", "(Ljava/math/BigInteger;Ljava/math/BigInteger;Ljava/math/BigInteger;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrime](https://developer.android.com/reference/java/security/spec/RSAOtherPrimeInfo.html#getPrime())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPrime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAOtherPrimeInfo", java.flags == PUBLIC | FINAL, .name == "getPrime", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAOtherPrimeInfo\0", "getPrime\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExponent](https://developer.android.com/reference/java/security/spec/RSAOtherPrimeInfo.html#getExponent())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getExponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAOtherPrimeInfo", java.flags == PUBLIC | FINAL, .name == "getExponent", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAOtherPrimeInfo\0", "getExponent\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCrtCoefficient](https://developer.android.com/reference/java/security/spec/RSAOtherPrimeInfo.html#getCrtCoefficient())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getCrtCoefficient<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/RSAOtherPrimeInfo", java.flags == PUBLIC | FINAL, .name == "getCrtCoefficient", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/RSAOtherPrimeInfo\0", "getCrtCoefficient\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
