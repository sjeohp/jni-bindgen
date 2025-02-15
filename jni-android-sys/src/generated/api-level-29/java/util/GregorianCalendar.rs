// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-GregorianCalendar"))]
__jni_bindgen! {
    /// public class [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html)
    ///
    /// Required feature: java-util-GregorianCalendar
    public class GregorianCalendar ("java/util/GregorianCalendar") extends crate::java::util::Calendar {

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(java.util.TimeZone))
        ///
        /// Required features: "java-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "java-util-TimeZone")))]
        pub fn new_TimeZone<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::TimeZone>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/TimeZone;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(Ljava/util/TimeZone;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn new_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(java.util.TimeZone,%20java.util.Locale))
        ///
        /// Required features: "java-util-Locale", "java-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale", feature = "java-util-TimeZone")))]
        pub fn new_TimeZone_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::TimeZone>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/TimeZone;Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(Ljava/util/TimeZone;Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(int,%20int,%20int))
        pub fn new_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(III)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(int,%20int,%20int,%20int,%20int))
        pub fn new_int_int_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IIIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(IIIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GregorianCalendar](https://developer.android.com/reference/java/util/GregorianCalendar.html#GregorianCalendar(int,%20int,%20int,%20int,%20int,%20int))
        pub fn new_int_int_int_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IIIIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "<init>\0", "(IIIIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setGregorianChange](https://developer.android.com/reference/java/util/GregorianCalendar.html#setGregorianChange(java.util.Date))
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn setGregorianChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "setGregorianChange", .descriptor == "(Ljava/util/Date;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "setGregorianChange\0", "(Ljava/util/Date;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGregorianChange](https://developer.android.com/reference/java/util/GregorianCalendar.html#getGregorianChange())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getGregorianChange<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC | FINAL, .name == "getGregorianChange", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getGregorianChange\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isLeapYear](https://developer.android.com/reference/java/util/GregorianCalendar.html#isLeapYear(int))
        pub fn isLeapYear<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "isLeapYear", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "isLeapYear\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCalendarType](https://developer.android.com/reference/java/util/GregorianCalendar.html#getCalendarType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getCalendarType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getCalendarType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getCalendarType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/util/GregorianCalendar.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/util/GregorianCalendar.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [add](https://developer.android.com/reference/java/util/GregorianCalendar.html#add(int,%20int))
        pub fn add<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "add", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "add\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [roll](https://developer.android.com/reference/java/util/GregorianCalendar.html#roll(int,%20boolean))
        pub fn roll_int_boolean<'env>(&'env self, arg0: i32, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "roll", .descriptor == "(IZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "roll\0", "(IZ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [roll](https://developer.android.com/reference/java/util/GregorianCalendar.html#roll(int,%20int))
        pub fn roll_int_int<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "roll", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "roll\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMinimum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getMinimum(int))
        pub fn getMinimum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getMinimum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getMinimum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMaximum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getMaximum(int))
        pub fn getMaximum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getMaximum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getMaximum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getGreatestMinimum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getGreatestMinimum(int))
        pub fn getGreatestMinimum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getGreatestMinimum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getGreatestMinimum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLeastMaximum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getLeastMaximum(int))
        pub fn getLeastMaximum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getLeastMaximum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getLeastMaximum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActualMinimum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getActualMinimum(int))
        pub fn getActualMinimum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getActualMinimum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getActualMinimum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActualMaximum](https://developer.android.com/reference/java/util/GregorianCalendar.html#getActualMaximum(int))
        pub fn getActualMaximum<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getActualMaximum", .descriptor == "(I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getActualMaximum\0", "(I)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/util/GregorianCalendar.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeZone](https://developer.android.com/reference/java/util/GregorianCalendar.html#getTimeZone())
        ///
        /// Required features: "java-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "java-util-TimeZone")))]
        pub fn getTimeZone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::TimeZone>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getTimeZone", .descriptor == "()Ljava/util/TimeZone;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getTimeZone\0", "()Ljava/util/TimeZone;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeZone](https://developer.android.com/reference/java/util/GregorianCalendar.html#setTimeZone(java.util.TimeZone))
        ///
        /// Required features: "java-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "java-util-TimeZone")))]
        pub fn setTimeZone<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::TimeZone>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "setTimeZone", .descriptor == "(Ljava/util/TimeZone;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "setTimeZone\0", "(Ljava/util/TimeZone;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isWeekDateSupported](https://developer.android.com/reference/java/util/GregorianCalendar.html#isWeekDateSupported())
        pub fn isWeekDateSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC | FINAL, .name == "isWeekDateSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "isWeekDateSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWeekYear](https://developer.android.com/reference/java/util/GregorianCalendar.html#getWeekYear())
        pub fn getWeekYear<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getWeekYear", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getWeekYear\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWeekDate](https://developer.android.com/reference/java/util/GregorianCalendar.html#setWeekDate(int,%20int,%20int))
        pub fn setWeekDate<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "setWeekDate", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "setWeekDate\0", "(III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWeeksInWeekYear](https://developer.android.com/reference/java/util/GregorianCalendar.html#getWeeksInWeekYear())
        pub fn getWeeksInWeekYear<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "getWeeksInWeekYear", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "getWeeksInWeekYear\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [computeFields](https://developer.android.com/reference/java/util/GregorianCalendar.html#computeFields())
        // fn computeFields<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/GregorianCalendar", java.flags == PROTECTED, .name == "computeFields", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "computeFields\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [computeTime](https://developer.android.com/reference/java/util/GregorianCalendar.html#computeTime())
        // fn computeTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/util/GregorianCalendar", java.flags == PROTECTED, .name == "computeTime", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "computeTime\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [toZonedDateTime](https://developer.android.com/reference/java/util/GregorianCalendar.html#toZonedDateTime())
        ///
        /// Required features: "java-time-ZonedDateTime"
        #[cfg(any(feature = "all", all(feature = "java-time-ZonedDateTime")))]
        pub fn toZonedDateTime<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::time::ZonedDateTime>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC, .name == "toZonedDateTime", .descriptor == "()Ljava/time/ZonedDateTime;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/GregorianCalendar\0", "toZonedDateTime\0", "()Ljava/time/ZonedDateTime;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [from](https://developer.android.com/reference/java/util/GregorianCalendar.html#from(java.time.ZonedDateTime))
        ///
        /// Required features: "java-time-ZonedDateTime", "java-util-GregorianCalendar"
        #[cfg(any(feature = "all", all(feature = "java-time-ZonedDateTime", feature = "java-util-GregorianCalendar")))]
        pub fn from<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::time::ZonedDateTime>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::GregorianCalendar>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/GregorianCalendar", java.flags == PUBLIC | STATIC, .name == "from", .descriptor == "(Ljava/time/ZonedDateTime;)Ljava/util/GregorianCalendar;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/util/GregorianCalendar\0", "from\0", "(Ljava/time/ZonedDateTime;)Ljava/util/GregorianCalendar;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [AD](https://developer.android.com/reference/java/util/GregorianCalendar.html#AD)
        pub const AD : i32 = 1;

        /// public static final [BC](https://developer.android.com/reference/java/util/GregorianCalendar.html#BC)
        pub const BC : i32 = 0;
    }
}
