// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-locks-AbstractOwnableSynchronizer"))]
__jni_bindgen! {
    /// public class [AbstractOwnableSynchronizer](https://developer.android.com/reference/java/util/concurrent/locks/AbstractOwnableSynchronizer.html)
    ///
    /// Required feature: java-util-concurrent-locks-AbstractOwnableSynchronizer
    public class AbstractOwnableSynchronizer ("java/util/concurrent/locks/AbstractOwnableSynchronizer") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        // // Not emitting: Non-public method
        // /// [AbstractOwnableSynchronizer](https://developer.android.com/reference/java/util/concurrent/locks/AbstractOwnableSynchronizer.html#AbstractOwnableSynchronizer())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::locks::AbstractOwnableSynchronizer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/locks/AbstractOwnableSynchronizer", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/AbstractOwnableSynchronizer\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [setExclusiveOwnerThread](https://developer.android.com/reference/java/util/concurrent/locks/AbstractOwnableSynchronizer.html#setExclusiveOwnerThread(java.lang.Thread))
        // ///
        // /// Required features: "java-lang-Thread"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Thread")))]
        // fn setExclusiveOwnerThread<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Thread>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/locks/AbstractOwnableSynchronizer", java.flags == PROTECTED | FINAL, .name == "setExclusiveOwnerThread", .descriptor == "(Ljava/lang/Thread;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/AbstractOwnableSynchronizer\0", "setExclusiveOwnerThread\0", "(Ljava/lang/Thread;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [getExclusiveOwnerThread](https://developer.android.com/reference/java/util/concurrent/locks/AbstractOwnableSynchronizer.html#getExclusiveOwnerThread())
        // ///
        // /// Required features: "java-lang-Thread"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Thread")))]
        // fn getExclusiveOwnerThread<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Thread>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/locks/AbstractOwnableSynchronizer", java.flags == PROTECTED | FINAL, .name == "getExclusiveOwnerThread", .descriptor == "()Ljava/lang/Thread;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/AbstractOwnableSynchronizer\0", "getExclusiveOwnerThread\0", "()Ljava/lang/Thread;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
