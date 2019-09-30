// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-locks-Lock"))]
__jni_bindgen! {
    /// public interface [Lock](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html)
    ///
    /// Required feature: java-util-concurrent-locks-Lock
    public interface Lock ("java/util/concurrent/locks/Lock") extends crate::java::lang::Object {

        /// [lock](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#lock())
        pub fn lock<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "lock", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "lock\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lockInterruptibly](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#lockInterruptibly())
        pub fn lockInterruptibly<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "lockInterruptibly", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "lockInterruptibly\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryLock](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#tryLock())
        pub fn tryLock<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "tryLock", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "tryLock\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryLock](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#tryLock(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-TimeUnit")))]
        pub fn tryLock_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "tryLock", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "tryLock\0", "(JLjava/util/concurrent/TimeUnit;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unlock](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#unlock())
        pub fn unlock<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "unlock", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "unlock\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newCondition](https://developer.android.com/reference/java/util/concurrent/locks/Lock.html#newCondition())
        ///
        /// Required features: "java-util-concurrent-locks-Condition"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-locks-Condition")))]
        pub fn newCondition<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::concurrent::locks::Condition>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/Lock", java.flags == PUBLIC | ABSTRACT, .name == "newCondition", .descriptor == "()Ljava/util/concurrent/locks/Condition;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/Lock\0", "newCondition\0", "()Ljava/util/concurrent/locks/Condition;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
