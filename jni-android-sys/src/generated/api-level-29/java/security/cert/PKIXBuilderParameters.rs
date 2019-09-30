// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-PKIXBuilderParameters"))]
__jni_bindgen! {
    /// public class [PKIXBuilderParameters](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html)
    ///
    /// Required feature: java-security-cert-PKIXBuilderParameters
    public class PKIXBuilderParameters ("java/security/cert/PKIXBuilderParameters") extends crate::java::security::cert::PKIXParameters {

        /// [PKIXBuilderParameters](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html#PKIXBuilderParameters(java.util.Set,%20java.security.cert.CertSelector))
        ///
        /// Required features: "java-security-cert-CertSelector", "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertSelector", feature = "java-util-Set")))]
        pub fn new_Set_CertSelector<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertSelector>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXBuilderParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXBuilderParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Set;Ljava/security/cert/CertSelector;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXBuilderParameters\0", "<init>\0", "(Ljava/util/Set;Ljava/security/cert/CertSelector;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PKIXBuilderParameters](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html#PKIXBuilderParameters(java.security.KeyStore,%20java.security.cert.CertSelector))
        ///
        /// Required features: "java-security-KeyStore", "java-security-cert-CertSelector"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyStore", feature = "java-security-cert-CertSelector")))]
        pub fn new_KeyStore_CertSelector<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyStore>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertSelector>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXBuilderParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXBuilderParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/KeyStore;Ljava/security/cert/CertSelector;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXBuilderParameters\0", "<init>\0", "(Ljava/security/KeyStore;Ljava/security/cert/CertSelector;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMaxPathLength](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html#setMaxPathLength(int))
        pub fn setMaxPathLength<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXBuilderParameters", java.flags == PUBLIC, .name == "setMaxPathLength", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXBuilderParameters\0", "setMaxPathLength\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaxPathLength](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html#getMaxPathLength())
        pub fn getMaxPathLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXBuilderParameters", java.flags == PUBLIC, .name == "getMaxPathLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXBuilderParameters\0", "getMaxPathLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/cert/PKIXBuilderParameters.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXBuilderParameters", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXBuilderParameters\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
