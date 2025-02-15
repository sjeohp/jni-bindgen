// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-OptionalDouble"))]
__jni_bindgen! {
    /// public final class [OptionalDouble](https://developer.android.com/reference/java/util/OptionalDouble.html)
    ///
    /// Required feature: java-util-OptionalDouble
    public final class OptionalDouble ("java/util/OptionalDouble") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [OptionalDouble](https://developer.android.com/reference/java/util/OptionalDouble.html#OptionalDouble())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::OptionalDouble>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/OptionalDouble", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [empty](https://developer.android.com/reference/java/util/OptionalDouble.html#empty())
        ///
        /// Required features: "java-util-OptionalDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-OptionalDouble")))]
        pub fn empty<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::OptionalDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC | STATIC, .name == "empty", .descriptor == "()Ljava/util/OptionalDouble;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/OptionalDouble\0", "empty\0", "()Ljava/util/OptionalDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [of](https://developer.android.com/reference/java/util/OptionalDouble.html#of(double))
        ///
        /// Required features: "java-util-OptionalDouble"
        #[cfg(any(feature = "all", all(feature = "java-util-OptionalDouble")))]
        pub fn of<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: f64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::OptionalDouble>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC | STATIC, .name == "of", .descriptor == "(D)Ljava/util/OptionalDouble;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/OptionalDouble\0", "of\0", "(D)Ljava/util/OptionalDouble;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAsDouble](https://developer.android.com/reference/java/util/OptionalDouble.html#getAsDouble())
        pub fn getAsDouble<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "getAsDouble", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "getAsDouble\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPresent](https://developer.android.com/reference/java/util/OptionalDouble.html#isPresent())
        pub fn isPresent<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "isPresent", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "isPresent\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ifPresent](https://developer.android.com/reference/java/util/OptionalDouble.html#ifPresent(java.util.function.DoubleConsumer))
        ///
        /// Required features: "java-util-function-DoubleConsumer"
        #[cfg(any(feature = "all", all(feature = "java-util-function-DoubleConsumer")))]
        pub fn ifPresent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::DoubleConsumer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "ifPresent", .descriptor == "(Ljava/util/function/DoubleConsumer;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "ifPresent\0", "(Ljava/util/function/DoubleConsumer;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [orElse](https://developer.android.com/reference/java/util/OptionalDouble.html#orElse(double))
        pub fn orElse<'env>(&'env self, arg0: f64) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "orElse", .descriptor == "(D)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "orElse\0", "(D)D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [orElseGet](https://developer.android.com/reference/java/util/OptionalDouble.html#orElseGet(java.util.function.DoubleSupplier))
        ///
        /// Required features: "java-util-function-DoubleSupplier"
        #[cfg(any(feature = "all", all(feature = "java-util-function-DoubleSupplier")))]
        pub fn orElseGet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::DoubleSupplier>>) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "orElseGet", .descriptor == "(Ljava/util/function/DoubleSupplier;)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "orElseGet\0", "(Ljava/util/function/DoubleSupplier;)D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [orElseThrow](https://developer.android.com/reference/java/util/OptionalDouble.html#orElseThrow(java.util.function.Supplier))
        ///
        /// Required features: "java-util-function-Supplier"
        #[cfg(any(feature = "all", all(feature = "java-util-function-Supplier")))]
        pub fn orElseThrow<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::Supplier>>) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "orElseThrow", .descriptor == "(Ljava/util/function/Supplier;)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "orElseThrow\0", "(Ljava/util/function/Supplier;)D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/util/OptionalDouble.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/OptionalDouble.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/util/OptionalDouble.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/OptionalDouble", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/OptionalDouble\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
