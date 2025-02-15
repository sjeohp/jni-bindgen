// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-spec-ECPrivateKeySpec"))]
__jni_bindgen! {
    /// public class [ECPrivateKeySpec](https://developer.android.com/reference/java/security/spec/ECPrivateKeySpec.html)
    ///
    /// Required feature: java-security-spec-ECPrivateKeySpec
    public class ECPrivateKeySpec ("java/security/spec/ECPrivateKeySpec") extends crate::java::lang::Object, implements crate::java::security::spec::KeySpec {

        /// [ECPrivateKeySpec](https://developer.android.com/reference/java/security/spec/ECPrivateKeySpec.html#ECPrivateKeySpec(java.math.BigInteger,%20java.security.spec.ECParameterSpec))
        ///
        /// Required features: "java-math-BigInteger", "java-security-spec-ECParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger", feature = "java-security-spec-ECParameterSpec")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::math::BigInteger>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::ECParameterSpec>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::spec::ECPrivateKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPrivateKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/math/BigInteger;Ljava/security/spec/ECParameterSpec;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPrivateKeySpec\0", "<init>\0", "(Ljava/math/BigInteger;Ljava/security/spec/ECParameterSpec;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getS](https://developer.android.com/reference/java/security/spec/ECPrivateKeySpec.html#getS())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getS<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPrivateKeySpec", java.flags == PUBLIC, .name == "getS", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPrivateKeySpec\0", "getS\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParams](https://developer.android.com/reference/java/security/spec/ECPrivateKeySpec.html#getParams())
        ///
        /// Required features: "java-security-spec-ECParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-ECParameterSpec")))]
        pub fn getParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::ECParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPrivateKeySpec", java.flags == PUBLIC, .name == "getParams", .descriptor == "()Ljava/security/spec/ECParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPrivateKeySpec\0", "getParams\0", "()Ljava/security/spec/ECParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
