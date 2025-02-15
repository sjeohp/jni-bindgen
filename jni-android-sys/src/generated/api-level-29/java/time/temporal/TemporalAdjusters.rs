// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-temporal-TemporalAdjusters"))]
__jni_bindgen! {
    /// public final class [TemporalAdjusters](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html)
    ///
    /// Required feature: java-time-temporal-TemporalAdjusters
    public final class TemporalAdjusters ("java/time/temporal/TemporalAdjusters") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [TemporalAdjusters](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#TemporalAdjusters())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjusters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/temporal/TemporalAdjusters", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/temporal/TemporalAdjusters\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [ofDateAdjuster](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#ofDateAdjuster(java.util.function.UnaryOperator))
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster", "java-util-function-UnaryOperator"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster", feature = "java-util-function-UnaryOperator")))]
        pub fn ofDateAdjuster<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::UnaryOperator>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "ofDateAdjuster", .descriptor == "(Ljava/util/function/UnaryOperator;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "ofDateAdjuster\0", "(Ljava/util/function/UnaryOperator;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [firstDayOfMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#firstDayOfMonth())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn firstDayOfMonth<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "firstDayOfMonth", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "firstDayOfMonth\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lastDayOfMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#lastDayOfMonth())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn lastDayOfMonth<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "lastDayOfMonth", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "lastDayOfMonth\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [firstDayOfNextMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#firstDayOfNextMonth())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn firstDayOfNextMonth<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "firstDayOfNextMonth", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "firstDayOfNextMonth\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [firstDayOfYear](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#firstDayOfYear())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn firstDayOfYear<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "firstDayOfYear", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "firstDayOfYear\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lastDayOfYear](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#lastDayOfYear())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn lastDayOfYear<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "lastDayOfYear", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "lastDayOfYear\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [firstDayOfNextYear](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#firstDayOfNextYear())
        ///
        /// Required features: "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn firstDayOfNextYear<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "firstDayOfNextYear", .descriptor == "()Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "firstDayOfNextYear\0", "()Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [firstInMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#firstInMonth(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn firstInMonth<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "firstInMonth", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "firstInMonth\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lastInMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#lastInMonth(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn lastInMonth<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "lastInMonth", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "lastInMonth\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dayOfWeekInMonth](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#dayOfWeekInMonth(int,%20java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn dayOfWeekInMonth<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "dayOfWeekInMonth", .descriptor == "(ILjava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "dayOfWeekInMonth\0", "(ILjava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [next](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#next(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn next<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "next", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "next\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [nextOrSame](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#nextOrSame(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn nextOrSame<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "nextOrSame", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "nextOrSame\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [previous](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#previous(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn previous<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "previous", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "previous\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [previousOrSame](https://developer.android.com/reference/java/time/temporal/TemporalAdjusters.html#previousOrSame(java.time.DayOfWeek))
        ///
        /// Required features: "java-time-DayOfWeek", "java-time-temporal-TemporalAdjuster"
        #[cfg(any(feature = "all", all(feature = "java-time-DayOfWeek", feature = "java-time-temporal-TemporalAdjuster")))]
        pub fn previousOrSame<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::DayOfWeek>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::temporal::TemporalAdjuster>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/temporal/TemporalAdjusters", java.flags == PUBLIC | STATIC, .name == "previousOrSame", .descriptor == "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/temporal/TemporalAdjusters\0", "previousOrSame\0", "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
