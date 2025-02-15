// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-atomic-AtomicLongArray"))]
__jni_bindgen! {
    /// public class [AtomicLongArray](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html)
    ///
    /// Required feature: java-util-concurrent-atomic-AtomicLongArray
    public class AtomicLongArray ("java/util/concurrent/atomic/AtomicLongArray") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        /// [AtomicLongArray](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#AtomicLongArray(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::atomic::AtomicLongArray>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [AtomicLongArray](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#AtomicLongArray(long%5B%5D))
        pub fn new_long_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::atomic::AtomicLongArray>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC, .name == "<init>", .descriptor == "([J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "<init>\0", "([J)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [length](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#length())
        pub fn length<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "length", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "length\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#get(int))
        pub fn get<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "get", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "get\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#set(int,%20long))
        pub fn set<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "set", .descriptor == "(IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "set\0", "(IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lazySet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#lazySet(int,%20long))
        pub fn lazySet<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "lazySet", .descriptor == "(IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "lazySet\0", "(IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndSet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndSet(int,%20long))
        pub fn getAndSet<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndSet", .descriptor == "(IJ)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndSet\0", "(IJ)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareAndSet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#compareAndSet(int,%20long,%20long))
        pub fn compareAndSet<'env>(&'env self, arg0: i32, arg1: i64, arg2: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "compareAndSet", .descriptor == "(IJJ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "compareAndSet\0", "(IJJ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [weakCompareAndSet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#weakCompareAndSet(int,%20long,%20long))
        pub fn weakCompareAndSet<'env>(&'env self, arg0: i32, arg1: i64, arg2: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "weakCompareAndSet", .descriptor == "(IJJ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "weakCompareAndSet\0", "(IJJ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndIncrement](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndIncrement(int))
        pub fn getAndIncrement<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndIncrement", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndIncrement\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndDecrement](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndDecrement(int))
        pub fn getAndDecrement<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndDecrement", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndDecrement\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndAdd](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndAdd(int,%20long))
        pub fn getAndAdd<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndAdd", .descriptor == "(IJ)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndAdd\0", "(IJ)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [incrementAndGet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#incrementAndGet(int))
        pub fn incrementAndGet<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "incrementAndGet", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "incrementAndGet\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [decrementAndGet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#decrementAndGet(int))
        pub fn decrementAndGet<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "decrementAndGet", .descriptor == "(I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "decrementAndGet\0", "(I)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addAndGet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#addAndGet(int,%20long))
        pub fn addAndGet<'env>(&'env self, arg0: i32, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC, .name == "addAndGet", .descriptor == "(IJ)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "addAndGet\0", "(IJ)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndUpdate](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndUpdate(int,%20java.util.function.LongUnaryOperator))
        ///
        /// Required features: "java-util-function-LongUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-LongUnaryOperator")))]
        pub fn getAndUpdate<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::LongUnaryOperator>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndUpdate", .descriptor == "(ILjava/util/function/LongUnaryOperator;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndUpdate\0", "(ILjava/util/function/LongUnaryOperator;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [updateAndGet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#updateAndGet(int,%20java.util.function.LongUnaryOperator))
        ///
        /// Required features: "java-util-function-LongUnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-LongUnaryOperator")))]
        pub fn updateAndGet<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::LongUnaryOperator>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "updateAndGet", .descriptor == "(ILjava/util/function/LongUnaryOperator;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "updateAndGet\0", "(ILjava/util/function/LongUnaryOperator;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAndAccumulate](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#getAndAccumulate(int,%20long,%20java.util.function.LongBinaryOperator))
        ///
        /// Required features: "java-util-function-LongBinaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-LongBinaryOperator")))]
        pub fn getAndAccumulate<'env>(&'env self, arg0: i32, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::LongBinaryOperator>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "getAndAccumulate", .descriptor == "(IJLjava/util/function/LongBinaryOperator;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "getAndAccumulate\0", "(IJLjava/util/function/LongBinaryOperator;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [accumulateAndGet](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#accumulateAndGet(int,%20long,%20java.util.function.LongBinaryOperator))
        ///
        /// Required features: "java-util-function-LongBinaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-util-function-LongBinaryOperator")))]
        pub fn accumulateAndGet<'env>(&'env self, arg0: i32, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::LongBinaryOperator>>) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC | FINAL, .name == "accumulateAndGet", .descriptor == "(IJLjava/util/function/LongBinaryOperator;)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "accumulateAndGet\0", "(IJLjava/util/function/LongBinaryOperator;)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/concurrent/atomic/AtomicLongArray.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/AtomicLongArray", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/AtomicLongArray\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
