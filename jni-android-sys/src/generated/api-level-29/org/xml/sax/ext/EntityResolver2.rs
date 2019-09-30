// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-xml-sax-ext-EntityResolver2"))]
__jni_bindgen! {
    /// public interface [EntityResolver2](https://developer.android.com/reference/org/xml/sax/ext/EntityResolver2.html)
    ///
    /// Required feature: org-xml-sax-ext-EntityResolver2
    public interface EntityResolver2 ("org/xml/sax/ext/EntityResolver2") extends crate::java::lang::Object, implements crate::org::xml::sax::EntityResolver {

        /// [getExternalSubset](https://developer.android.com/reference/org/xml/sax/ext/EntityResolver2.html#getExternalSubset(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "org-xml-sax-InputSource"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-xml-sax-InputSource")))]
        pub fn getExternalSubset<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::xml::sax::InputSource>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/ext/EntityResolver2", java.flags == PUBLIC | ABSTRACT, .name == "getExternalSubset", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Lorg/xml/sax/InputSource;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/ext/EntityResolver2\0", "getExternalSubset\0", "(Ljava/lang/String;Ljava/lang/String;)Lorg/xml/sax/InputSource;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [resolveEntity](https://developer.android.com/reference/org/xml/sax/ext/EntityResolver2.html#resolveEntity(java.lang.String,%20java.lang.String,%20java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "org-xml-sax-InputSource"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "org-xml-sax-InputSource")))]
        pub fn resolveEntity<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::org::xml::sax::InputSource>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/ext/EntityResolver2", java.flags == PUBLIC | ABSTRACT, .name == "resolveEntity", .descriptor == "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lorg/xml/sax/InputSource;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/ext/EntityResolver2\0", "resolveEntity\0", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lorg/xml/sax/InputSource;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
