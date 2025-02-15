// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-xml-transform-sax-TemplatesHandler"))]
__jni_bindgen! {
    /// public interface [TemplatesHandler](https://developer.android.com/reference/javax/xml/transform/sax/TemplatesHandler.html)
    ///
    /// Required feature: javax-xml-transform-sax-TemplatesHandler
    public interface TemplatesHandler ("javax/xml/transform/sax/TemplatesHandler") extends crate::java::lang::Object, implements crate::org::xml::sax::ContentHandler {

        /// [getTemplates](https://developer.android.com/reference/javax/xml/transform/sax/TemplatesHandler.html#getTemplates())
        ///
        /// Required features: "javax-xml-transform-Templates"
        #[cfg(any(feature = "all", all(feature = "javax-xml-transform-Templates")))]
        pub fn getTemplates<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::xml::transform::Templates>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/sax/TemplatesHandler", java.flags == PUBLIC | ABSTRACT, .name == "getTemplates", .descriptor == "()Ljavax/xml/transform/Templates;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/sax/TemplatesHandler\0", "getTemplates\0", "()Ljavax/xml/transform/Templates;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSystemId](https://developer.android.com/reference/javax/xml/transform/sax/TemplatesHandler.html#setSystemId(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSystemId<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/sax/TemplatesHandler", java.flags == PUBLIC | ABSTRACT, .name == "setSystemId", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/sax/TemplatesHandler\0", "setSystemId\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/javax/xml/transform/sax/TemplatesHandler.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/xml/transform/sax/TemplatesHandler", java.flags == PUBLIC | ABSTRACT, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/xml/transform/sax/TemplatesHandler\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
