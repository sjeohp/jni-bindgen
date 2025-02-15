// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-Consumer"))]
__jni_bindgen! {
    /// public interface [Consumer](https://developer.android.com/reference/java/util/function/Consumer.html)
    ///
    /// Required feature: java-util-function-Consumer
    public interface Consumer ("java/util/function/Consumer") extends crate::java::lang::Object {

        /// [accept](https://developer.android.com/reference/java/util/function/Consumer.html#accept(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn accept<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/Consumer", java.flags == PUBLIC | ABSTRACT, .name == "accept", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/Consumer\0", "accept\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [andThen](https://developer.android.com/reference/java/util/function/Consumer.html#andThen(java.util.function.Consumer))
        ///
        /// Required features: "java-util-function-Consumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-Consumer")))]
        pub fn andThen<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::Consumer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::Consumer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/Consumer", java.flags == PUBLIC, .name == "andThen", .descriptor == "(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/Consumer\0", "andThen\0", "(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
