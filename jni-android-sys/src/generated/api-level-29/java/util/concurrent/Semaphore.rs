// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-Semaphore"))]
__jni_bindgen! {
    /// public class [Semaphore](https://developer.android.com/reference/java/util/concurrent/Semaphore.html)
    ///
    /// Required feature: java-util-concurrent-Semaphore
    public class Semaphore ("java/util/concurrent/Semaphore") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        /// [Semaphore](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#Semaphore(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::Semaphore>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Semaphore](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#Semaphore(int,%20boolean))
        pub fn new_int_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::Semaphore>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "<init>\0", "(IZ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [acquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#acquire())
        pub fn acquire<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "acquire", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "acquire\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [acquireUninterruptibly](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#acquireUninterruptibly())
        pub fn acquireUninterruptibly<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "acquireUninterruptibly", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "acquireUninterruptibly\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryAcquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#tryAcquire())
        pub fn tryAcquire<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "tryAcquire", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "tryAcquire\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryAcquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#tryAcquire(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-TimeUnit")))]
        pub fn tryAcquire_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "tryAcquire", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "tryAcquire\0", "(JLjava/util/concurrent/TimeUnit;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [release](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#release())
        pub fn release<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "release", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "release\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [acquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#acquire(int))
        pub fn acquire_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "acquire", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "acquire\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [acquireUninterruptibly](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#acquireUninterruptibly(int))
        pub fn acquireUninterruptibly_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "acquireUninterruptibly", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "acquireUninterruptibly\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryAcquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#tryAcquire(int))
        pub fn tryAcquire_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "tryAcquire", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "tryAcquire\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tryAcquire](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#tryAcquire(int,%20long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-TimeUnit")))]
        pub fn tryAcquire_int_long_TimeUnit<'env>(&'env self, arg0: i32, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "tryAcquire", .descriptor == "(IJLjava/util/concurrent/TimeUnit;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "tryAcquire\0", "(IJLjava/util/concurrent/TimeUnit;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [release](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#release(int))
        pub fn release_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "release", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "release\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [availablePermits](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#availablePermits())
        pub fn availablePermits<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "availablePermits", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "availablePermits\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [drainPermits](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#drainPermits())
        pub fn drainPermits<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "drainPermits", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "drainPermits\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [reducePermits](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#reducePermits(int))
        // fn reducePermits<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/Semaphore", java.flags == PROTECTED, .name == "reducePermits", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "reducePermits\0", "(I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isFair](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#isFair())
        pub fn isFair<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "isFair", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "isFair\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasQueuedThreads](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#hasQueuedThreads())
        pub fn hasQueuedThreads<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC | FINAL, .name == "hasQueuedThreads", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "hasQueuedThreads\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getQueueLength](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#getQueueLength())
        pub fn getQueueLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC | FINAL, .name == "getQueueLength", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "getQueueLength\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [getQueuedThreads](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#getQueuedThreads())
        // ///
        // /// Required features: "java-util-Collection"
        // #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        // fn getQueuedThreads<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/Semaphore", java.flags == PROTECTED, .name == "getQueuedThreads", .descriptor == "()Ljava/util/Collection;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "getQueuedThreads\0", "()Ljava/util/Collection;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [toString](https://developer.android.com/reference/java/util/concurrent/Semaphore.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/Semaphore", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/Semaphore\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
