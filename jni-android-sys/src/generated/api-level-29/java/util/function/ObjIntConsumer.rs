// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-function-ObjIntConsumer"))]
__jni_bindgen! {
    /// public interface [ObjIntConsumer](https://developer.android.com/reference/java/util/function/ObjIntConsumer.html)
    ///
    /// Required feature: java-util-function-ObjIntConsumer
    public interface ObjIntConsumer ("java/util/function/ObjIntConsumer") extends crate::java::lang::Object {

        /// [accept](https://developer.android.com/reference/java/util/function/ObjIntConsumer.html#accept(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn accept<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/function/ObjIntConsumer", java.flags == PUBLIC | ABSTRACT, .name == "accept", .descriptor == "(Ljava/lang/Object;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/function/ObjIntConsumer\0", "accept\0", "(Ljava/lang/Object;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
