// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-AlgorithmConstraints"))]
__jni_bindgen! {
    /// public interface [AlgorithmConstraints](https://developer.android.com/reference/java/security/AlgorithmConstraints.html)
    ///
    /// Required feature: java-security-AlgorithmConstraints
    public interface AlgorithmConstraints ("java/security/AlgorithmConstraints") extends crate::java::lang::Object {

        /// [permits](https://developer.android.com/reference/java/security/AlgorithmConstraints.html#permits(java.util.Set,%20java.lang.String,%20java.security.AlgorithmParameters))
        ///
        /// Required features: "java-lang-String", "java-security-AlgorithmParameters", "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-AlgorithmParameters", feature = "java-util-Set")))]
        pub fn permits_Set_String_AlgorithmParameters<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::AlgorithmParameters>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/AlgorithmConstraints", java.flags == PUBLIC | ABSTRACT, .name == "permits", .descriptor == "(Ljava/util/Set;Ljava/lang/String;Ljava/security/AlgorithmParameters;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmConstraints\0", "permits\0", "(Ljava/util/Set;Ljava/lang/String;Ljava/security/AlgorithmParameters;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [permits](https://developer.android.com/reference/java/security/AlgorithmConstraints.html#permits(java.util.Set,%20java.security.Key))
        ///
        /// Required features: "java-security-Key", "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-security-Key", feature = "java-util-Set")))]
        pub fn permits_Set_Key<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Key>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/AlgorithmConstraints", java.flags == PUBLIC | ABSTRACT, .name == "permits", .descriptor == "(Ljava/util/Set;Ljava/security/Key;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmConstraints\0", "permits\0", "(Ljava/util/Set;Ljava/security/Key;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [permits](https://developer.android.com/reference/java/security/AlgorithmConstraints.html#permits(java.util.Set,%20java.lang.String,%20java.security.Key,%20java.security.AlgorithmParameters))
        ///
        /// Required features: "java-lang-String", "java-security-AlgorithmParameters", "java-security-Key", "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-AlgorithmParameters", feature = "java-security-Key", feature = "java-util-Set")))]
        pub fn permits_Set_String_Key_AlgorithmParameters<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Key>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::AlgorithmParameters>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/AlgorithmConstraints", java.flags == PUBLIC | ABSTRACT, .name == "permits", .descriptor == "(Ljava/util/Set;Ljava/lang/String;Ljava/security/Key;Ljava/security/AlgorithmParameters;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmConstraints\0", "permits\0", "(Ljava/util/Set;Ljava/lang/String;Ljava/security/Key;Ljava/security/AlgorithmParameters;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
