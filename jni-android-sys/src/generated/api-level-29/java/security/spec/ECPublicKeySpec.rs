// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-spec-ECPublicKeySpec"))]
__jni_bindgen! {
    /// public class [ECPublicKeySpec](https://developer.android.com/reference/java/security/spec/ECPublicKeySpec.html)
    ///
    /// Required feature: java-security-spec-ECPublicKeySpec
    public class ECPublicKeySpec ("java/security/spec/ECPublicKeySpec") extends crate::java::lang::Object, implements crate::java::security::spec::KeySpec {

        /// [ECPublicKeySpec](https://developer.android.com/reference/java/security/spec/ECPublicKeySpec.html#ECPublicKeySpec(java.security.spec.ECPoint,%20java.security.spec.ECParameterSpec))
        ///
        /// Required features: "java-security-spec-ECParameterSpec", "java-security-spec-ECPoint"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-ECParameterSpec", feature = "java-security-spec-ECPoint")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::ECPoint>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::ECParameterSpec>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::spec::ECPublicKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPublicKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/spec/ECPoint;Ljava/security/spec/ECParameterSpec;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPublicKeySpec\0", "<init>\0", "(Ljava/security/spec/ECPoint;Ljava/security/spec/ECParameterSpec;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getW](https://developer.android.com/reference/java/security/spec/ECPublicKeySpec.html#getW())
        ///
        /// Required features: "java-security-spec-ECPoint"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-ECPoint")))]
        pub fn getW<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::ECPoint>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPublicKeySpec", java.flags == PUBLIC, .name == "getW", .descriptor == "()Ljava/security/spec/ECPoint;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPublicKeySpec\0", "getW\0", "()Ljava/security/spec/ECPoint;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParams](https://developer.android.com/reference/java/security/spec/ECPublicKeySpec.html#getParams())
        ///
        /// Required features: "java-security-spec-ECParameterSpec"
        #[cfg(any(feature = "all", all(feature = "java-security-spec-ECParameterSpec")))]
        pub fn getParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::spec::ECParameterSpec>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/ECPublicKeySpec", java.flags == PUBLIC, .name == "getParams", .descriptor == "()Ljava/security/spec/ECParameterSpec;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/ECPublicKeySpec\0", "getParams\0", "()Ljava/security/spec/ECParameterSpec;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
