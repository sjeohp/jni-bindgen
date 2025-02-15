// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-Calendar_Builder"))]
__jni_bindgen! {
    /// public class [Calendar.Builder](https://developer.android.com/reference/java/util/Calendar.Builder.html)
    ///
    /// Required feature: java-util-Calendar_Builder
    public class Calendar_Builder ("java/util/Calendar$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/java/util/Calendar.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInstant](https://developer.android.com/reference/java/util/Calendar.Builder.html#setInstant(long))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setInstant_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setInstant", .descriptor == "(J)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setInstant\0", "(J)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInstant](https://developer.android.com/reference/java/util/Calendar.Builder.html#setInstant(java.util.Date))
        ///
        /// Required features: "java-util-Calendar_Builder", "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder", feature = "java-util-Date")))]
        pub fn setInstant_Date<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setInstant", .descriptor == "(Ljava/util/Date;)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setInstant\0", "(Ljava/util/Date;)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/java/util/Calendar.Builder.html#set(int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn set<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "set", .descriptor == "(II)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "set\0", "(II)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFields](https://developer.android.com/reference/java/util/Calendar.Builder.html#setFields(int...))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setFields<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC | VARARGS, .name == "setFields", .descriptor == "([I)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setFields\0", "([I)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDate](https://developer.android.com/reference/java/util/Calendar.Builder.html#setDate(int,%20int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setDate<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setDate", .descriptor == "(III)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setDate\0", "(III)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeOfDay](https://developer.android.com/reference/java/util/Calendar.Builder.html#setTimeOfDay(int,%20int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setTimeOfDay_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setTimeOfDay", .descriptor == "(III)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setTimeOfDay\0", "(III)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeOfDay](https://developer.android.com/reference/java/util/Calendar.Builder.html#setTimeOfDay(int,%20int,%20int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setTimeOfDay_int_int_int_int<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setTimeOfDay", .descriptor == "(IIII)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setTimeOfDay\0", "(IIII)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWeekDate](https://developer.android.com/reference/java/util/Calendar.Builder.html#setWeekDate(int,%20int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setWeekDate<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setWeekDate", .descriptor == "(III)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setWeekDate\0", "(III)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeZone](https://developer.android.com/reference/java/util/Calendar.Builder.html#setTimeZone(java.util.TimeZone))
        ///
        /// Required features: "java-util-Calendar_Builder", "java-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder", feature = "java-util-TimeZone")))]
        pub fn setTimeZone<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::TimeZone>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setTimeZone", .descriptor == "(Ljava/util/TimeZone;)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setTimeZone\0", "(Ljava/util/TimeZone;)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLenient](https://developer.android.com/reference/java/util/Calendar.Builder.html#setLenient(boolean))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setLenient<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setLenient", .descriptor == "(Z)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setLenient\0", "(Z)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCalendarType](https://developer.android.com/reference/java/util/Calendar.Builder.html#setCalendarType(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Calendar_Builder")))]
        pub fn setCalendarType<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setCalendarType", .descriptor == "(Ljava/lang/String;)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setCalendarType\0", "(Ljava/lang/String;)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLocale](https://developer.android.com/reference/java/util/Calendar.Builder.html#setLocale(java.util.Locale))
        ///
        /// Required features: "java-util-Calendar_Builder", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder", feature = "java-util-Locale")))]
        pub fn setLocale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setLocale", .descriptor == "(Ljava/util/Locale;)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setLocale\0", "(Ljava/util/Locale;)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWeekDefinition](https://developer.android.com/reference/java/util/Calendar.Builder.html#setWeekDefinition(int,%20int))
        ///
        /// Required features: "java-util-Calendar_Builder"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar_Builder")))]
        pub fn setWeekDefinition<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "setWeekDefinition", .descriptor == "(II)Ljava/util/Calendar$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "setWeekDefinition\0", "(II)Ljava/util/Calendar$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/java/util/Calendar.Builder.html#build())
        ///
        /// Required features: "java-util-Calendar"
        #[cfg(any(feature = "all", all(feature = "java-util-Calendar")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Calendar>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/Calendar$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Ljava/util/Calendar;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/Calendar$Builder\0", "build\0", "()Ljava/util/Calendar;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
