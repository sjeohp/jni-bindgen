// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-xml-sax-SAXNotRecognizedException"))]
__jni_bindgen! {
    /// public class [SAXNotRecognizedException](https://developer.android.com/reference/org/xml/sax/SAXNotRecognizedException.html)
    ///
    /// Required feature: org-xml-sax-SAXNotRecognizedException
    public class SAXNotRecognizedException ("org/xml/sax/SAXNotRecognizedException") extends crate::org::xml::sax::SAXException {

        /// [SAXNotRecognizedException](https://developer.android.com/reference/org/xml/sax/SAXNotRecognizedException.html#SAXNotRecognizedException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::org::xml::sax::SAXNotRecognizedException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/SAXNotRecognizedException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/SAXNotRecognizedException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SAXNotRecognizedException](https://developer.android.com/reference/org/xml/sax/SAXNotRecognizedException.html#SAXNotRecognizedException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::org::xml::sax::SAXNotRecognizedException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/SAXNotRecognizedException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/SAXNotRecognizedException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
