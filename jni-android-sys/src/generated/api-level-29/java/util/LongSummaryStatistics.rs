// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-LongSummaryStatistics"))]
__jni_bindgen! {
    /// public class [LongSummaryStatistics](https://developer.android.com/reference/java/util/LongSummaryStatistics.html)
    ///
    /// Required feature: java-util-LongSummaryStatistics
    public class LongSummaryStatistics ("java/util/LongSummaryStatistics") extends crate::java::lang::Object, implements crate::java::util::function::LongConsumer, crate::java::util::function::IntConsumer {

        /// [LongSummaryStatistics](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#LongSummaryStatistics())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::LongSummaryStatistics>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [accept](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#accept(int))
        pub fn accept_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC, .name == "accept", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "accept\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [accept](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#accept(long))
        pub fn accept_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC, .name == "accept", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "accept\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [combine](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#combine(java.util.LongSummaryStatistics))
        ///
        /// Required features: "java-util-LongSummaryStatistics"
        #[cfg(any(feature = "all", all(feature = "java-util-LongSummaryStatistics")))]
        pub fn combine<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::LongSummaryStatistics>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC, .name == "combine", .descriptor == "(Ljava/util/LongSummaryStatistics;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "combine\0", "(Ljava/util/LongSummaryStatistics;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCount](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#getCount())
        pub fn getCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC | FINAL, .name == "getCount", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "getCount\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSum](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#getSum())
        pub fn getSum<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC | FINAL, .name == "getSum", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "getSum\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMin](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#getMin())
        pub fn getMin<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC | FINAL, .name == "getMin", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "getMin\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMax](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#getMax())
        pub fn getMax<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC | FINAL, .name == "getMax", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "getMax\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAverage](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#getAverage())
        pub fn getAverage<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC | FINAL, .name == "getAverage", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "getAverage\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/LongSummaryStatistics.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/LongSummaryStatistics", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/LongSummaryStatistics\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
