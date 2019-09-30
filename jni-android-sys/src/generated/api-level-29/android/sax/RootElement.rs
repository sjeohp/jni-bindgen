// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-sax-RootElement"))]
__jni_bindgen! {
    /// public class [RootElement](https://developer.android.com/reference/android/sax/RootElement.html)
    ///
    /// Required feature: android-sax-RootElement
    public class RootElement ("android/sax/RootElement") extends crate::android::sax::Element {

        /// [RootElement](https://developer.android.com/reference/android/sax/RootElement.html#RootElement(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::sax::RootElement>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/sax/RootElement", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/sax/RootElement\0", "<init>\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [RootElement](https://developer.android.com/reference/android/sax/RootElement.html#RootElement(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::sax::RootElement>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/sax/RootElement", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/sax/RootElement\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getContentHandler](https://developer.android.com/reference/android/sax/RootElement.html#getContentHandler())
        ///
        /// Required features: "org-xml-sax-ContentHandler"
        #[cfg(any(feature = "all", all(feature = "org-xml-sax-ContentHandler")))]
        pub fn getContentHandler<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::xml::sax::ContentHandler>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/sax/RootElement", java.flags == PUBLIC, .name == "getContentHandler", .descriptor == "()Lorg/xml/sax/ContentHandler;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/sax/RootElement\0", "getContentHandler\0", "()Lorg/xml/sax/ContentHandler;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
