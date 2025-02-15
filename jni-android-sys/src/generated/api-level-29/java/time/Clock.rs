// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-time-Clock"))]
__jni_bindgen! {
    /// public class [Clock](https://developer.android.com/reference/java/time/Clock.html)
    ///
    /// Required feature: java-time-Clock
    public class Clock ("java/time/Clock") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Clock](https://developer.android.com/reference/java/time/Clock.html#Clock())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::time::Clock>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/time/Clock", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [systemUTC](https://developer.android.com/reference/java/time/Clock.html#systemUTC())
        ///
        /// Required features: "java-time-Clock"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock")))]
        pub fn systemUTC<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "systemUTC", .descriptor == "()Ljava/time/Clock;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "systemUTC\0", "()Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [systemDefaultZone](https://developer.android.com/reference/java/time/Clock.html#systemDefaultZone())
        ///
        /// Required features: "java-time-Clock"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock")))]
        pub fn systemDefaultZone<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "systemDefaultZone", .descriptor == "()Ljava/time/Clock;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "systemDefaultZone\0", "()Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [system](https://developer.android.com/reference/java/time/Clock.html#system(java.time.ZoneId))
        ///
        /// Required features: "java-time-Clock", "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-ZoneId")))]
        pub fn system<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZoneId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "system", .descriptor == "(Ljava/time/ZoneId;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "system\0", "(Ljava/time/ZoneId;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tickSeconds](https://developer.android.com/reference/java/time/Clock.html#tickSeconds(java.time.ZoneId))
        ///
        /// Required features: "java-time-Clock", "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-ZoneId")))]
        pub fn tickSeconds<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZoneId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "tickSeconds", .descriptor == "(Ljava/time/ZoneId;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "tickSeconds\0", "(Ljava/time/ZoneId;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tickMinutes](https://developer.android.com/reference/java/time/Clock.html#tickMinutes(java.time.ZoneId))
        ///
        /// Required features: "java-time-Clock", "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-ZoneId")))]
        pub fn tickMinutes<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZoneId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "tickMinutes", .descriptor == "(Ljava/time/ZoneId;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "tickMinutes\0", "(Ljava/time/ZoneId;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [tick](https://developer.android.com/reference/java/time/Clock.html#tick(java.time.Clock,%20java.time.Duration))
        ///
        /// Required features: "java-time-Clock", "java-time-Duration"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-Duration")))]
        pub fn tick<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::Clock>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::Duration>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "tick", .descriptor == "(Ljava/time/Clock;Ljava/time/Duration;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "tick\0", "(Ljava/time/Clock;Ljava/time/Duration;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fixed](https://developer.android.com/reference/java/time/Clock.html#fixed(java.time.Instant,%20java.time.ZoneId))
        ///
        /// Required features: "java-time-Clock", "java-time-Instant", "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-Instant", feature = "java-time-ZoneId")))]
        pub fn fixed<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::Instant>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZoneId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "fixed", .descriptor == "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "fixed\0", "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [offset](https://developer.android.com/reference/java/time/Clock.html#offset(java.time.Clock,%20java.time.Duration))
        ///
        /// Required features: "java-time-Clock", "java-time-Duration"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-Duration")))]
        pub fn offset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::Clock>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::Duration>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | STATIC, .name == "offset", .descriptor == "(Ljava/time/Clock;Ljava/time/Duration;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/time/Clock\0", "offset\0", "(Ljava/time/Clock;Ljava/time/Duration;)Ljava/time/Clock;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getZone](https://developer.android.com/reference/java/time/Clock.html#getZone())
        ///
        /// Required features: "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-ZoneId")))]
        pub fn getZone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::ZoneId>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | ABSTRACT, .name == "getZone", .descriptor == "()Ljava/time/ZoneId;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "getZone\0", "()Ljava/time/ZoneId;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [withZone](https://developer.android.com/reference/java/time/Clock.html#withZone(java.time.ZoneId))
        ///
        /// Required features: "java-time-Clock", "java-time-ZoneId"
        #[cfg(any(feature = "all", all(feature = "java-time-Clock", feature = "java-time-ZoneId")))]
        pub fn withZone<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZoneId>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Clock>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | ABSTRACT, .name == "withZone", .descriptor == "(Ljava/time/ZoneId;)Ljava/time/Clock;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "withZone\0", "(Ljava/time/ZoneId;)Ljava/time/Clock;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [millis](https://developer.android.com/reference/java/time/Clock.html#millis())
        pub fn millis<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC, .name == "millis", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "millis\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [instant](https://developer.android.com/reference/java/time/Clock.html#instant())
        ///
        /// Required features: "java-time-Instant"
        #[cfg(any(feature = "all", all(feature = "java-time-Instant")))]
        pub fn instant<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::Instant>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC | ABSTRACT, .name == "instant", .descriptor == "()Ljava/time/Instant;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "instant\0", "()Ljava/time/Instant;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/time/Clock.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/time/Clock.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/time/Clock", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/time/Clock\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
