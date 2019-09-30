// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-transform-Result"))]
__jni_bindgen! {
    /// public interface [Result](https://developer.android.com/reference/javax/xml/transform/Result.html)
    ///
    /// Required feature: javax-xml-transform-Result
    public interface Result ("javax/xml/transform/Result") extends crate::java::lang::Object {

        /// [setSystemId](https://developer.android.com/reference/javax/xml/transform/Result.html#setSystemId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSystemId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/Result", java.flags == PUBLIC | ABSTRACT, .name == "setSystemId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/Result\0", "setSystemId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/javax/xml/transform/Result.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/Result", java.flags == PUBLIC | ABSTRACT, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/Result\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [PI_DISABLE_OUTPUT_ESCAPING](https://developer.android.com/reference/javax/xml/transform/Result.html#PI_DISABLE_OUTPUT_ESCAPING)
        pub const PI_DISABLE_OUTPUT_ESCAPING : &'static str = "javax.xml.transform.disable-output-escaping";

        /// public static final [PI_ENABLE_OUTPUT_ESCAPING](https://developer.android.com/reference/javax/xml/transform/Result.html#PI_ENABLE_OUTPUT_ESCAPING)
        pub const PI_ENABLE_OUTPUT_ESCAPING : &'static str = "javax.xml.transform.enable-output-escaping";
    }
}
