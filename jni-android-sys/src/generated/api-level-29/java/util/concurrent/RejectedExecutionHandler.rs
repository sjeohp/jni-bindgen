// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-RejectedExecutionHandler"))]
__jni_bindgen! {
    /// public interface [RejectedExecutionHandler](https://developer.android.com/reference/java/util/concurrent/RejectedExecutionHandler.html)
    ///
    /// Required feature: java-util-concurrent-RejectedExecutionHandler
    public interface RejectedExecutionHandler ("java/util/concurrent/RejectedExecutionHandler") extends crate::java::lang::Object {

        /// [rejectedExecution](https://developer.android.com/reference/java/util/concurrent/RejectedExecutionHandler.html#rejectedExecution(java.lang.Runnable,%20java.util.concurrent.ThreadPoolExecutor))
        ///
        /// Required features: "java-lang-Runnable", "java-util-concurrent-ThreadPoolExecutor"
        #[cfg(any(feature = "all", all(feature = "java-lang-Runnable", feature = "java-util-concurrent-ThreadPoolExecutor")))]
        pub fn rejectedExecution<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::ThreadPoolExecutor>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/RejectedExecutionHandler", java.flags == PUBLIC | ABSTRACT, .name == "rejectedExecution", .descriptor == "(Ljava/lang/Runnable;Ljava/util/concurrent/ThreadPoolExecutor;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/RejectedExecutionHandler\0", "rejectedExecution\0", "(Ljava/lang/Runnable;Ljava/util/concurrent/ThreadPoolExecutor;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
