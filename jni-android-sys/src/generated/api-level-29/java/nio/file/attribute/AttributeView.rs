// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-file-attribute-AttributeView"))]
__jni_bindgen! {
    /// public interface [AttributeView](https://developer.android.com/reference/java/nio/file/attribute/AttributeView.html)
    ///
    /// Required feature: java-nio-file-attribute-AttributeView
    public interface AttributeView ("java/nio/file/attribute/AttributeView") extends crate::java::lang::Object {

        /// [name](https://developer.android.com/reference/java/nio/file/attribute/AttributeView.html#name())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn name<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/file/attribute/AttributeView", java.flags == PUBLIC | ABSTRACT, .name == "name", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/file/attribute/AttributeView\0", "name\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
