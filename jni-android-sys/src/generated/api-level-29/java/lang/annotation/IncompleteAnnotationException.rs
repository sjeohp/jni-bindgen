// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-annotation-IncompleteAnnotationException"))]
__jni_bindgen! {
    /// public class [IncompleteAnnotationException](https://developer.android.com/reference/java/lang/annotation/IncompleteAnnotationException.html)
    ///
    /// Required feature: java-lang-annotation-IncompleteAnnotationException
    public class IncompleteAnnotationException ("java/lang/annotation/IncompleteAnnotationException") extends crate::java::lang::RuntimeException {

        /// [IncompleteAnnotationException](https://developer.android.com/reference/java/lang/annotation/IncompleteAnnotationException.html#IncompleteAnnotationException(java.lang.Class,%20java.lang.String))
        ///
        /// Required features: "java-lang-Class", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::annotation::IncompleteAnnotationException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/annotation/IncompleteAnnotationException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/Class;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/annotation/IncompleteAnnotationException\0", "<init>\0", "(Ljava/lang/Class;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [annotationType](https://developer.android.com/reference/java/lang/annotation/IncompleteAnnotationException.html#annotationType())
        ///
        /// Required features: "java-lang-Class"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class")))]
        pub fn annotationType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/annotation/IncompleteAnnotationException", java.flags == PUBLIC, .name == "annotationType", .descriptor == "()Ljava/lang/Class;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/annotation/IncompleteAnnotationException\0", "annotationType\0", "()Ljava/lang/Class;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [elementName](https://developer.android.com/reference/java/lang/annotation/IncompleteAnnotationException.html#elementName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn elementName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/annotation/IncompleteAnnotationException", java.flags == PUBLIC, .name == "elementName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/annotation/IncompleteAnnotationException\0", "elementName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
