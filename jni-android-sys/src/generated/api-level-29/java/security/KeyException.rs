// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-KeyException"))]
__jni_bindgen! {
    /// public class [KeyException](https://developer.android.com/reference/java/security/KeyException.html)
    ///
    /// Required feature: java-security-KeyException
    public class KeyException ("java/security/KeyException") extends crate::java::security::GeneralSecurityException {

        /// [KeyException](https://developer.android.com/reference/java/security/KeyException.html#KeyException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::KeyException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [KeyException](https://developer.android.com/reference/java/security/KeyException.html#KeyException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::KeyException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [KeyException](https://developer.android.com/reference/java/security/KeyException.html#KeyException(java.lang.String,%20java.lang.Throwable))
        ///
        /// Required features: "java-lang-String", "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-lang-Throwable")))]
        pub fn new_String_Throwable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::KeyException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/lang/Throwable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyException\0", "<init>\0", "(Ljava/lang/String;Ljava/lang/Throwable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [KeyException](https://developer.android.com/reference/java/security/KeyException.html#KeyException(java.lang.Throwable))
        ///
        /// Required features: "java-lang-Throwable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Throwable")))]
        pub fn new_Throwable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Throwable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::KeyException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/KeyException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Throwable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/KeyException\0", "<init>\0", "(Ljava/lang/Throwable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
