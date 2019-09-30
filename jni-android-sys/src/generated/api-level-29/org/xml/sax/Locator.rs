// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-xml-sax-Locator"))]
__jni_bindgen! {
    /// public interface [Locator](https://developer.android.com/reference/org/xml/sax/Locator.html)
    ///
    /// Required feature: org-xml-sax-Locator
    public interface Locator ("org/xml/sax/Locator") extends crate::java::lang::Object {

        /// [getPublicId](https://developer.android.com/reference/org/xml/sax/Locator.html#getPublicId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getPublicId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/Locator", java.flags == PUBLIC | ABSTRACT, .name == "getPublicId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/Locator\0", "getPublicId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSystemId](https://developer.android.com/reference/org/xml/sax/Locator.html#getSystemId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSystemId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/Locator", java.flags == PUBLIC | ABSTRACT, .name == "getSystemId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/Locator\0", "getSystemId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLineNumber](https://developer.android.com/reference/org/xml/sax/Locator.html#getLineNumber())
        pub fn getLineNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/Locator", java.flags == PUBLIC | ABSTRACT, .name == "getLineNumber", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/Locator\0", "getLineNumber\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getColumnNumber](https://developer.android.com/reference/org/xml/sax/Locator.html#getColumnNumber())
        pub fn getColumnNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/xml/sax/Locator", java.flags == PUBLIC | ABSTRACT, .name == "getColumnNumber", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/xml/sax/Locator\0", "getColumnNumber\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
