// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-Certificate_CertificateRep"))]
__jni_bindgen! {
    /// public class [Certificate.CertificateRep](https://developer.android.com/reference/java/security/cert/Certificate.CertificateRep.html)
    ///
    /// Required feature: java-security-cert-Certificate_CertificateRep
    public class Certificate_CertificateRep ("java/security/cert/Certificate$CertificateRep") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        // // Not emitting: Non-public method
        // /// [CertificateRep](https://developer.android.com/reference/java/security/cert/Certificate.CertificateRep.html#CertificateRep(java.lang.String,%20byte%5B%5D))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::Certificate_CertificateRep>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/Certificate$CertificateRep", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;[B)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/Certificate$CertificateRep\0", "<init>\0", "(Ljava/lang/String;[B)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [readResolve](https://developer.android.com/reference/java/security/cert/Certificate.CertificateRep.html#readResolve())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn readResolve<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/Certificate$CertificateRep", java.flags == PROTECTED, .name == "readResolve", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/Certificate$CertificateRep\0", "readResolve\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
