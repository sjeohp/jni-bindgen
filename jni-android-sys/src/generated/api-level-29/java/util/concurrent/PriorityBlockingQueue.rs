// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-PriorityBlockingQueue"))]
__jni_bindgen! {
    /// public class [PriorityBlockingQueue](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html)
    ///
    /// Required feature: java-util-concurrent-PriorityBlockingQueue
    public class PriorityBlockingQueue ("java/util/concurrent/PriorityBlockingQueue") extends crate::java::util::AbstractQueue, implements crate::java::util::concurrent::BlockingQueue, crate::java::io::Serializable {

        /// [PriorityBlockingQueue](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#PriorityBlockingQueue())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::PriorityBlockingQueue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PriorityBlockingQueue](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#PriorityBlockingQueue(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::PriorityBlockingQueue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PriorityBlockingQueue](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#PriorityBlockingQueue(int,%20java.util.Comparator))
        ///
        /// Required features: "java-util-Comparator"
        #[cfg(any(feature = "all", all(feature = "java-util-Comparator")))]
        pub fn new_int_Comparator<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Comparator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::PriorityBlockingQueue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "<init>", .descriptor == "(ILjava/util/Comparator;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "<init>\0", "(ILjava/util/Comparator;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PriorityBlockingQueue](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#PriorityBlockingQueue(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn new_Collection<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::PriorityBlockingQueue>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Collection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "<init>\0", "(Ljava/util/Collection;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#add(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn add<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "add", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "add\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [offer](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#offer(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn offer_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "offer", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "offer\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#put(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn put<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "put", .descriptor == "(Ljava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "put\0", "(Ljava/lang/Object;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [offer](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#offer(java.lang.Object,%20long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-lang-Object", "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-concurrent-TimeUnit")))]
        pub fn offer_Object_long_TimeUnit<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i64, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "offer", .descriptor == "(Ljava/lang/Object;JLjava/util/concurrent/TimeUnit;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "offer\0", "(Ljava/lang/Object;JLjava/util/concurrent/TimeUnit;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [poll](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#poll())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn poll<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "poll", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "poll\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [take](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#take())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn take<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "take", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "take\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [poll](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#poll(long,%20java.util.concurrent.TimeUnit))
        ///
        /// Required features: "java-lang-Object", "java-util-concurrent-TimeUnit"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-util-concurrent-TimeUnit")))]
        pub fn poll_long_TimeUnit<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::TimeUnit>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "poll", .descriptor == "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "poll\0", "(JLjava/util/concurrent/TimeUnit;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [peek](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#peek())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn peek<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "peek", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "peek\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [comparator](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#comparator())
        ///
        /// Required features: "java-util-Comparator"
        #[cfg(any(feature = "all", all(feature = "java-util-Comparator")))]
        pub fn comparator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Comparator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "comparator", .descriptor == "()Ljava/util/Comparator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "comparator\0", "()Ljava/util/Comparator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [size](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#size())
        pub fn size<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "size", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "size\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remainingCapacity](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#remainingCapacity())
        pub fn remainingCapacity<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "remainingCapacity", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "remainingCapacity\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [remove](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#remove(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn remove<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "remove", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "remove\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [contains](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#contains(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn contains<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "contains", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "contains\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [drainTo](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#drainTo(java.util.Collection))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn drainTo_Collection<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "drainTo", .descriptor == "(Ljava/util/Collection;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "drainTo\0", "(Ljava/util/Collection;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [drainTo](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#drainTo(java.util.Collection,%20int))
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn drainTo_Collection_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Collection>>, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "drainTo", .descriptor == "(Ljava/util/Collection;I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "drainTo\0", "(Ljava/util/Collection;I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toArray](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#toArray())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn toArray<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "toArray", .descriptor == "()[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "toArray\0", "()[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toArray](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#toArray(java.lang.Object%5B%5D))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn toArray_Object_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "toArray", .descriptor == "([Ljava/lang/Object;)[Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "toArray\0", "([Ljava/lang/Object;)[Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [iterator](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#iterator())
        ///
        /// Required features: "java-util-Iterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Iterator")))]
        pub fn iterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Iterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "iterator", .descriptor == "()Ljava/util/Iterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "iterator\0", "()Ljava/util/Iterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [spliterator](https://developer.android.com/reference/java/util/concurrent/PriorityBlockingQueue.html#spliterator())
        ///
        /// Required features: "java-util-Spliterator"
        #[cfg(any(feature = "all", all(feature = "java-util-Spliterator")))]
        pub fn spliterator<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Spliterator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/PriorityBlockingQueue", java.flags == PUBLIC, .name == "spliterator", .descriptor == "()Ljava/util/Spliterator;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/PriorityBlockingQueue\0", "spliterator\0", "()Ljava/util/Spliterator;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
