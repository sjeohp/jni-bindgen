// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-LongFunction"))]
__jni_bindgen! {
    /// public interface [LongFunction](https://developer.android.com/reference/java/util/function/LongFunction.html)
    ///
    /// Required feature: java-util-function-LongFunction
    public interface LongFunction ("java/util/function/LongFunction") extends crate::java::lang::Object {

        /// [apply](https://developer.android.com/reference/java/util/function/LongFunction.html#apply(long))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn apply<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/LongFunction", java.flags == PUBLIC | ABSTRACT, .name == "apply", .descriptor == "(J)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/LongFunction\0", "apply\0", "(J)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
