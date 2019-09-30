// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-util-BuddhistCalendar"))]
__jni_bindgen! {
    /// public class [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html)
    ///
    /// Required feature: android-icu-util-BuddhistCalendar
    public class BuddhistCalendar ("android/icu/util/BuddhistCalendar") extends crate::android::icu::util::GregorianCalendar {

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(android.icu.util.TimeZone))
        ///
        /// Required features: "android-icu-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-TimeZone")))]
        pub fn new_TimeZone<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::TimeZone>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/util/TimeZone;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Landroid/icu/util/TimeZone;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn new_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(android.icu.util.ULocale))
        ///
        /// Required features: "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale")))]
        pub fn new_ULocale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/util/ULocale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Landroid/icu/util/ULocale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(android.icu.util.TimeZone,%20java.util.Locale))
        ///
        /// Required features: "android-icu-util-TimeZone", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-TimeZone", feature = "java-util-Locale")))]
        pub fn new_TimeZone_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::TimeZone>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/util/TimeZone;Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Landroid/icu/util/TimeZone;Ljava/util/Locale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(android.icu.util.TimeZone,%20android.icu.util.ULocale))
        ///
        /// Required features: "android-icu-util-TimeZone", "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-TimeZone", feature = "android-icu-util-ULocale")))]
        pub fn new_TimeZone_ULocale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::TimeZone>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/util/TimeZone;Landroid/icu/util/ULocale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Landroid/icu/util/TimeZone;Landroid/icu/util/ULocale;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(java.util.Date))
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn new_Date<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Date;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(Ljava/util/Date;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(int,%20int,%20int))
        pub fn new_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(III)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BuddhistCalendar](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BuddhistCalendar(int,%20int,%20int,%20int,%20int,%20int))
        pub fn new_int_int_int_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::BuddhistCalendar>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "<init>", .descriptor == "(IIIIII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "<init>\0", "(IIIIII)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [handleGetExtendedYear](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#handleGetExtendedYear())
        // fn handleGetExtendedYear<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/util/BuddhistCalendar", java.flags == PROTECTED, .name == "handleGetExtendedYear", .descriptor == "()I"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "handleGetExtendedYear\0", "()I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [handleComputeMonthStart](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#handleComputeMonthStart(int,%20int,%20boolean))
        // fn handleComputeMonthStart<'env>(&'env self, arg0: i32, arg1: i32, arg2: bool) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/util/BuddhistCalendar", java.flags == PROTECTED, .name == "handleComputeMonthStart", .descriptor == "(IIZ)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "handleComputeMonthStart\0", "(IIZ)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [handleComputeFields](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#handleComputeFields(int))
        // fn handleComputeFields<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/util/BuddhistCalendar", java.flags == PROTECTED, .name == "handleComputeFields", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "handleComputeFields\0", "(I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [handleGetLimit](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#handleGetLimit(int,%20int))
        // fn handleGetLimit<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/util/BuddhistCalendar", java.flags == PROTECTED, .name == "handleGetLimit", .descriptor == "(II)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "handleGetLimit\0", "(II)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getType](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#getType())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/BuddhistCalendar", java.flags == PUBLIC, .name == "getType", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/BuddhistCalendar\0", "getType\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [BE](https://developer.android.com/reference/android/icu/util/BuddhistCalendar.html#BE)
        pub const BE : i32 = 0;
    }
}
