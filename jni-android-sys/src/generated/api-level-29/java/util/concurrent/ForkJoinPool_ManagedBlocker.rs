// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-ForkJoinPool_ManagedBlocker"))]
__jni_bindgen! {
    /// public interface [ForkJoinPool.ManagedBlocker](https://developer.android.com/reference/java/util/concurrent/ForkJoinPool.ManagedBlocker.html)
    ///
    /// Required feature: java-util-concurrent-ForkJoinPool_ManagedBlocker
    public interface ForkJoinPool_ManagedBlocker ("java/util/concurrent/ForkJoinPool$ManagedBlocker") extends crate::java::lang::Object {

        /// [block](https://developer.android.com/reference/java/util/concurrent/ForkJoinPool.ManagedBlocker.html#block())
        pub fn block<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/ForkJoinPool$ManagedBlocker", java.flags == PUBLIC | ABSTRACT, .name == "block", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/ForkJoinPool$ManagedBlocker\0", "block\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isReleasable](https://developer.android.com/reference/java/util/concurrent/ForkJoinPool.ManagedBlocker.html#isReleasable())
        pub fn isReleasable<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/ForkJoinPool$ManagedBlocker", java.flags == PUBLIC | ABSTRACT, .name == "isReleasable", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/ForkJoinPool$ManagedBlocker\0", "isReleasable\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
