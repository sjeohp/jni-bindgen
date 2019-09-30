// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-concurrent-atomic-DoubleAdder"))]
__jni_bindgen! {
    /// public class [DoubleAdder](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html)
    ///
    /// Required feature: java-util-concurrent-atomic-DoubleAdder
    public class DoubleAdder ("java/util/concurrent/atomic/DoubleAdder") extends crate::java::lang::Number, implements crate::java::io::Serializable {

        /// [DoubleAdder](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#DoubleAdder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::concurrent::atomic::DoubleAdder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#add(double))
        pub fn add<'env>(&'env self, arg0: f64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "add", .descriptor == "(D)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "add\0", "(D)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sum](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#sum())
        pub fn sum<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "sum", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "sum\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sumThenReset](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#sumThenReset())
        pub fn sumThenReset<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "sumThenReset", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "sumThenReset\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [doubleValue](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#doubleValue())
        pub fn doubleValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "doubleValue", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "doubleValue\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [longValue](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#longValue())
        pub fn longValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "longValue", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "longValue\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [intValue](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#intValue())
        pub fn intValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "intValue", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "intValue\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [floatValue](https://developer.android.com/reference/java/util/concurrent/atomic/DoubleAdder.html#floatValue())
        pub fn floatValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/concurrent/atomic/DoubleAdder", java.flags == PUBLIC, .name == "floatValue", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/concurrent/atomic/DoubleAdder\0", "floatValue\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
