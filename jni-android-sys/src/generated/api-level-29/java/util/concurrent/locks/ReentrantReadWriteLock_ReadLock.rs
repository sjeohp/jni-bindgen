// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-locks-ReentrantReadWriteLock_ReadLock"))]
__jni_bindgen! {
    /// public class [ReentrantReadWriteLock.ReadLock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html)
    ///
    /// Required feature: java-util-concurrent-locks-ReentrantReadWriteLock_ReadLock
    public class ReentrantReadWriteLock_ReadLock ("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock") extends crate::java::lang::Object, implements crate::java::util::concurrent::locks::Lock, crate::java::io::Serializable {

        // // Not emitting: Non-public method
        // /// [ReadLock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#ReadLock(java.util.concurrent.locks.ReentrantReadWriteLock))
        // ///
        // /// Required features: "java-util-concurrent-locks-ReentrantReadWriteLock"
        // #[cfg(any(feature = "all", all(feature = "java-util-concurrent-locks-ReentrantReadWriteLock")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::locks::ReentrantReadWriteLock>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::locks::ReentrantReadWriteLock_ReadLock>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/util/concurrent/locks/ReentrantReadWriteLock;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "<init>\0", "(Ljava/util/concurrent/locks/ReentrantReadWriteLock;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [lock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#lock())
        pub fn lock<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "lock", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "lock\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lockInterruptibly](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#lockInterruptibly())
        pub fn lockInterruptibly<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "lockInterruptibly", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "lockInterruptibly\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryLock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#tryLock())
        pub fn tryLock<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "tryLock", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "tryLock\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryLock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#tryLock(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-TimeUnit")))]
        pub fn tryLock_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "tryLock", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "tryLock\0", "(JLjava/util/concurrent/TimeUnit;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unlock](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#unlock())
        pub fn unlock<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "unlock", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "unlock\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newCondition](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#newCondition())
        ///
        /// Required features: "java-util-concurrent-locks-Condition"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-locks-Condition")))]
        pub fn newCondition<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::concurrent::locks::Condition>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "newCondition", .descriptor == "()Ljava/util/concurrent/locks/Condition;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "newCondition\0", "()Ljava/util/concurrent/locks/Condition;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/concurrent/locks/ReentrantReadWriteLock.ReadLock.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/ReentrantReadWriteLock$ReadLock\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
