// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-interfaces-ECPublicKey"))]
__jni_bindgen! {
    /// public interface [ECPublicKey](https://developer.android.com/reference/java/security/interfaces/ECPublicKey.html)
    ///
    /// Required feature: java-security-interfaces-ECPublicKey
    public interface ECPublicKey ("java/security/interfaces/ECPublicKey") extends crate::java::lang::Object, implements crate::java::security::PublicKey, crate::java::security::interfaces::ECKey {

        /// [getW](https://developer.android.com/reference/java/security/interfaces/ECPublicKey.html#getW())
        ///
        /// Required features: "java-security-spec-ECPoint"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-ECPoint")))]
        pub fn getW<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::ECPoint>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/interfaces/ECPublicKey", java.flags == PUBLIC | ABSTRACT, .name == "getW", .descriptor == "()Ljava/security/spec/ECPoint;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/interfaces/ECPublicKey\0", "getW\0", "()Ljava/security/spec/ECPoint;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [serialVersionUID](https://developer.android.com/reference/java/security/interfaces/ECPublicKey.html#serialVersionUID)
        pub const serialVersionUID : i64 = -3314988629879632826i64;
    }
}
