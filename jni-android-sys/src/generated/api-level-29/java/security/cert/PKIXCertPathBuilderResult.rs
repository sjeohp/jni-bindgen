// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-PKIXCertPathBuilderResult"))]
__jni_bindgen! {
    /// public class [PKIXCertPathBuilderResult](https://developer.android.com/reference/java/security/cert/PKIXCertPathBuilderResult.html)
    ///
    /// Required feature: java-security-cert-PKIXCertPathBuilderResult
    public class PKIXCertPathBuilderResult ("java/security/cert/PKIXCertPathBuilderResult") extends crate::java::security::cert::PKIXCertPathValidatorResult, implements crate::java::security::cert::CertPathBuilderResult {

        /// [PKIXCertPathBuilderResult](https://developer.android.com/reference/java/security/cert/PKIXCertPathBuilderResult.html#PKIXCertPathBuilderResult(java.security.cert.CertPath,%20java.security.cert.TrustAnchor,%20java.security.cert.PolicyNode,%20java.security.PublicKey))
        ///
        /// Required features: "java-security-PublicKey", "java-security-cert-CertPath", "java-security-cert-PolicyNode", "java-security-cert-TrustAnchor"
        #[cfg(any(feature = "all", all(feature = "java-security-PublicKey", feature = "java-security-cert-CertPath", feature = "java-security-cert-PolicyNode", feature = "java-security-cert-TrustAnchor")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertPath>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::TrustAnchor>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::PolicyNode>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PublicKey>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXCertPathBuilderResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXCertPathBuilderResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/cert/CertPath;Ljava/security/cert/TrustAnchor;Ljava/security/cert/PolicyNode;Ljava/security/PublicKey;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXCertPathBuilderResult\0", "<init>\0", "(Ljava/security/cert/CertPath;Ljava/security/cert/TrustAnchor;Ljava/security/cert/PolicyNode;Ljava/security/PublicKey;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertPath](https://developer.android.com/reference/java/security/cert/PKIXCertPathBuilderResult.html#getCertPath())
        ///
        /// Required features: "java-security-cert-CertPath"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPath")))]
        pub fn getCertPath<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPath>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXCertPathBuilderResult", java.flags == PUBLIC, .name == "getCertPath", .descriptor == "()Ljava/security/cert/CertPath;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXCertPathBuilderResult\0", "getCertPath\0", "()Ljava/security/cert/CertPath;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/cert/PKIXCertPathBuilderResult.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXCertPathBuilderResult", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXCertPathBuilderResult\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
