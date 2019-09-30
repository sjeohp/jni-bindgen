// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-CyclicBarrier"))]
__jni_bindgen! {
    /// public class [CyclicBarrier](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html)
    ///
    /// Required feature: java-util-concurrent-CyclicBarrier
    public class CyclicBarrier ("java/util/concurrent/CyclicBarrier") extends crate::java::lang::Object {

        /// [CyclicBarrier](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#CyclicBarrier(int,%20java.lang.Runnable))
        ///
        /// Required features: "java-lang-Runnable"
        #[cfg(any(feature = "all", all(feature = "java-lang-Runnable")))]
        pub fn new_int_Runnable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Runnable>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::CyclicBarrier>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "<init>", .descriptor == "(ILjava/lang/Runnable;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "<init>\0", "(ILjava/lang/Runnable;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [CyclicBarrier](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#CyclicBarrier(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::CyclicBarrier>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParties](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#getParties())
        pub fn getParties<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "getParties", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "getParties\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [await](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#await())
        pub fn r#await<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "await", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "await\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [await](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#await(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-util-concurrent-TimeUnit")))]
        pub fn await_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "await", .descriptor == "(JLjava/util/concurrent/TimeUnit;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "await\0", "(JLjava/util/concurrent/TimeUnit;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isBroken](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#isBroken())
        pub fn isBroken<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "isBroken", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "isBroken\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumberWaiting](https://developer.android.com/reference/java/util/concurrent/CyclicBarrier.html#getNumberWaiting())
        pub fn getNumberWaiting<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/CyclicBarrier", java.flags == PUBLIC, .name == "getNumberWaiting", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/CyclicBarrier\0", "getNumberWaiting\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
