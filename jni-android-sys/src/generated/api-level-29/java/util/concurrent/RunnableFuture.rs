// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-RunnableFuture"))]
__jni_bindgen! {
    /// public interface [RunnableFuture](https://developer.android.com/reference/java/util/concurrent/RunnableFuture.html)
    ///
    /// Required feature: java-util-concurrent-RunnableFuture
    public interface RunnableFuture ("java/util/concurrent/RunnableFuture") extends crate::java::lang::Object, implements crate::java::lang::Runnable, crate::java::util::concurrent::Future {

        /// [run](https://developer.android.com/reference/java/util/concurrent/RunnableFuture.html#run())
        pub fn run<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/RunnableFuture", java.flags == PUBLIC | ABSTRACT, .name == "run", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/RunnableFuture\0", "run\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
