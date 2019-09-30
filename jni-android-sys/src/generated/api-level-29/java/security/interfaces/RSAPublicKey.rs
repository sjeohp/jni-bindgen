// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-interfaces-RSAPublicKey"))]
__jni_bindgen! {
    /// public interface [RSAPublicKey](https://developer.android.com/reference/java/security/interfaces/RSAPublicKey.html)
    ///
    /// Required feature: java-security-interfaces-RSAPublicKey
    public interface RSAPublicKey ("java/security/interfaces/RSAPublicKey") extends crate::java::lang::Object, implements crate::java::security::PublicKey, crate::java::security::interfaces::RSAKey {

        /// [getPublicExponent](https://developer.android.com/reference/java/security/interfaces/RSAPublicKey.html#getPublicExponent())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getPublicExponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/RSAPublicKey", java.flags == PUBLIC | ABSTRACT, .name == "getPublicExponent", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/RSAPublicKey\0", "getPublicExponent\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [serialVersionUID](https://developer.android.com/reference/java/security/interfaces/RSAPublicKey.html#serialVersionUID)
        pub const serialVersionUID : i64 = -8727434096241101194i64;
    }
}
