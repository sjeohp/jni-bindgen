// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-security-AttestedKeyPair"))]
__jni_bindgen! {
    /// public final class [AttestedKeyPair](https://developer.android.com/reference/android/security/AttestedKeyPair.html)
    ///
    /// Required feature: android-security-AttestedKeyPair
    public final class AttestedKeyPair ("android/security/AttestedKeyPair") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [AttestedKeyPair](https://developer.android.com/reference/android/security/AttestedKeyPair.html#AttestedKeyPair(java.security.KeyPair,%20java.security.cert.Certificate%5B%5D))
        // ///
        // /// Required features: "java-security-KeyPair", "java-security-cert-Certificate"
        // #[cfg(any(feature = "all", all(feature = "java-security-KeyPair", feature = "java-security-cert-Certificate")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyPair>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::cert::Certificate, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::security::AttestedKeyPair>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/security/AttestedKeyPair", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/security/KeyPair;[Ljava/security/cert/Certificate;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/AttestedKeyPair\0", "<init>\0", "(Ljava/security/KeyPair;[Ljava/security/cert/Certificate;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getKeyPair](https://developer.android.com/reference/android/security/AttestedKeyPair.html#getKeyPair())
        ///
        /// Required features: "java-security-KeyPair"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyPair")))]
        pub fn getKeyPair<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::KeyPair>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/AttestedKeyPair", java.flags == PUBLIC, .name == "getKeyPair", .descriptor == "()Ljava/security/KeyPair;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/AttestedKeyPair\0", "getKeyPair\0", "()Ljava/security/KeyPair;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAttestationRecord](https://developer.android.com/reference/android/security/AttestedKeyPair.html#getAttestationRecord())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getAttestationRecord<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/security/AttestedKeyPair", java.flags == PUBLIC, .name == "getAttestationRecord", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/security/AttestedKeyPair\0", "getAttestationRecord\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
