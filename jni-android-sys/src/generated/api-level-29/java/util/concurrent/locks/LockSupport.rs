// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-locks-LockSupport"))]
__jni_bindgen! {
    /// public class [LockSupport](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html)
    ///
    /// Required feature: java-util-concurrent-locks-LockSupport
    public class LockSupport ("java/util/concurrent/locks/LockSupport") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [LockSupport](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#LockSupport())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::locks::LockSupport>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/concurrent/locks/LockSupport", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/locks/LockSupport\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [unpark](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#unpark(java.lang.Thread))
        ///
        /// Required features: "java-lang-Thread"
        #[cfg(any(feature = "all", all(feature = "java-lang-Thread")))]
        pub fn unpark<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Thread>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "unpark", .descriptor == "(Ljava/lang/Thread;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "unpark\0", "(Ljava/lang/Thread;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [park](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#park(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn park_Object<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "park", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "park\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parkNanos](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#parkNanos(java.lang.Object,%20long))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn parkNanos_Object_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "parkNanos", .descriptor == "(Ljava/lang/Object;J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "parkNanos\0", "(Ljava/lang/Object;J)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parkUntil](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#parkUntil(java.lang.Object,%20long))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn parkUntil_Object_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "parkUntil", .descriptor == "(Ljava/lang/Object;J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "parkUntil\0", "(Ljava/lang/Object;J)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBlocker](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#getBlocker(java.lang.Thread))
        ///
        /// Required features: "java-lang-Object", "java-lang-Thread"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-Thread")))]
        pub fn getBlocker<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Thread>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "getBlocker", .descriptor == "(Ljava/lang/Thread;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "getBlocker\0", "(Ljava/lang/Thread;)Ljava/lang/Object;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [park](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#park())
        pub fn park<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "park", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "park\0", "()V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parkNanos](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#parkNanos(long))
        pub fn parkNanos_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "parkNanos", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "parkNanos\0", "(J)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parkUntil](https://developer.android.com/reference/java/util/concurrent/locks/LockSupport.html#parkUntil(long))
        pub fn parkUntil_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/locks/LockSupport", java.flags == PUBLIC | STATIC, .name == "parkUntil", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/concurrent/locks/LockSupport\0", "parkUntil\0", "(J)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
