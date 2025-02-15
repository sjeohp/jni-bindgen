// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-spec-EncodedKeySpec"))]
__jni_bindgen! {
    /// public class [EncodedKeySpec](https://developer.android.com/reference/java/security/spec/EncodedKeySpec.html)
    ///
    /// Required feature: java-security-spec-EncodedKeySpec
    public class EncodedKeySpec ("java/security/spec/EncodedKeySpec") extends crate::java::lang::Object, implements crate::java::security::spec::KeySpec {

        /// [EncodedKeySpec](https://developer.android.com/reference/java/security/spec/EncodedKeySpec.html#EncodedKeySpec(byte%5B%5D))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::spec::EncodedKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/EncodedKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "([B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/EncodedKeySpec\0", "<init>\0", "([B)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncoded](https://developer.android.com/reference/java/security/spec/EncodedKeySpec.html#getEncoded())
        pub fn getEncoded<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/EncodedKeySpec", java.flags == PUBLIC, .name == "getEncoded", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/EncodedKeySpec\0", "getEncoded\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFormat](https://developer.android.com/reference/java/security/spec/EncodedKeySpec.html#getFormat())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getFormat<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/spec/EncodedKeySpec", java.flags == PUBLIC | ABSTRACT, .name == "getFormat", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/spec/EncodedKeySpec\0", "getFormat\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
