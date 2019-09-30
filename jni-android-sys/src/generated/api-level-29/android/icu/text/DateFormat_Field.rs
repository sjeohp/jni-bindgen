// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
__jni_bindgen! {
    /// public class [DateFormat.Field](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html)
    ///
    /// Required feature: android-icu-text-DateFormat_Field
    public class DateFormat_Field ("android/icu/text/DateFormat$Field") extends crate::java::text::Format_Field {

        // // Not emitting: Non-public method
        // /// [Field](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#Field(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/DateFormat$Field", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateFormat$Field\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [ofCalendarField](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#ofCalendarField(int))
        ///
        /// Required features: "android-icu-text-DateFormat_Field"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateFormat_Field")))]
        pub fn ofCalendarField<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateFormat$Field", java.flags == PUBLIC | STATIC, .name == "ofCalendarField", .descriptor == "(I)Landroid/icu/text/DateFormat$Field;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateFormat$Field\0", "ofCalendarField\0", "(I)Landroid/icu/text/DateFormat$Field;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCalendarField](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#getCalendarField())
        pub fn getCalendarField<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateFormat$Field", java.flags == PUBLIC, .name == "getCalendarField", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateFormat$Field\0", "getCalendarField\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [readResolve](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#readResolve())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // fn readResolve<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/DateFormat$Field", java.flags == PROTECTED, .name == "readResolve", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateFormat$Field\0", "readResolve\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [AM_PM](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#AM_PM)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn AM_PM<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "AM_PM\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [AM_PM_MIDNIGHT_NOON](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#AM_PM_MIDNIGHT_NOON)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn AM_PM_MIDNIGHT_NOON<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "AM_PM_MIDNIGHT_NOON\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAY_OF_MONTH](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#DAY_OF_MONTH)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn DAY_OF_MONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "DAY_OF_MONTH\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAY_OF_WEEK](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#DAY_OF_WEEK)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn DAY_OF_WEEK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "DAY_OF_WEEK\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAY_OF_WEEK_IN_MONTH](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#DAY_OF_WEEK_IN_MONTH)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn DAY_OF_WEEK_IN_MONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "DAY_OF_WEEK_IN_MONTH\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAY_OF_YEAR](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#DAY_OF_YEAR)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn DAY_OF_YEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "DAY_OF_YEAR\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DOW_LOCAL](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#DOW_LOCAL)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn DOW_LOCAL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "DOW_LOCAL\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ERA](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#ERA)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn ERA<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "ERA\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [EXTENDED_YEAR](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#EXTENDED_YEAR)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn EXTENDED_YEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "EXTENDED_YEAR\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FLEXIBLE_DAY_PERIOD](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#FLEXIBLE_DAY_PERIOD)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn FLEXIBLE_DAY_PERIOD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "FLEXIBLE_DAY_PERIOD\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOUR0](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#HOUR0)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn HOUR0<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "HOUR0\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOUR1](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#HOUR1)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn HOUR1<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "HOUR1\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOUR_OF_DAY0](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#HOUR_OF_DAY0)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn HOUR_OF_DAY0<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "HOUR_OF_DAY0\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOUR_OF_DAY1](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#HOUR_OF_DAY1)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn HOUR_OF_DAY1<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "HOUR_OF_DAY1\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [JULIAN_DAY](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#JULIAN_DAY)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn JULIAN_DAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "JULIAN_DAY\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MILLISECOND](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#MILLISECOND)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn MILLISECOND<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "MILLISECOND\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MILLISECONDS_IN_DAY](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#MILLISECONDS_IN_DAY)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn MILLISECONDS_IN_DAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "MILLISECONDS_IN_DAY\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MINUTE](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#MINUTE)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn MINUTE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "MINUTE\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MONTH](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#MONTH)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn MONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "MONTH\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [QUARTER](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#QUARTER)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn QUARTER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "QUARTER\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SECOND](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#SECOND)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn SECOND<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "SECOND\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [TIME_ZONE](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#TIME_ZONE)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn TIME_ZONE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "TIME_ZONE\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WEEK_OF_MONTH](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#WEEK_OF_MONTH)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn WEEK_OF_MONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "WEEK_OF_MONTH\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WEEK_OF_YEAR](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#WEEK_OF_YEAR)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn WEEK_OF_YEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "WEEK_OF_YEAR\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [YEAR](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#YEAR)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn YEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "YEAR\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [YEAR_WOY](https://developer.android.com/reference/android/icu/text/DateFormat.Field.html#YEAR_WOY)
        ///
        /// Required feature: android-icu-text-DateFormat_Field
        #[cfg(any(feature = "all", feature = "android-icu-text-DateFormat_Field"))]
        pub fn YEAR_WOY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat_Field>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/DateFormat$Field\0", "YEAR_WOY\0", "Landroid/icu/text/DateFormat$Field;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
