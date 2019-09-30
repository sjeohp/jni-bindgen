// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-format-DateFormat"))]
__jni_bindgen! {
    /// public class [DateFormat](https://developer.android.com/reference/android/text/format/DateFormat.html)
    ///
    /// Required feature: android-text-format-DateFormat
    public class DateFormat ("android/text/format/DateFormat") extends crate::java::lang::Object {

        /// [DateFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#DateFormat())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::format::DateFormat>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/format/DateFormat\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [is24HourFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#is24HourFormat(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn is24HourFormat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "is24HourFormat", .descriptor == "(Landroid/content/Context;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "is24HourFormat\0", "(Landroid/content/Context;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBestDateTimePattern](https://developer.android.com/reference/android/text/format/DateFormat.html#getBestDateTimePattern(java.util.Locale,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getBestDateTimePattern<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getBestDateTimePattern", .descriptor == "(Ljava/util/Locale;Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getBestDateTimePattern\0", "(Ljava/util/Locale;Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#getTimeFormat(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-text-DateFormat"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-text-DateFormat")))]
        pub fn getTimeFormat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::DateFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getTimeFormat", .descriptor == "(Landroid/content/Context;)Ljava/text/DateFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getTimeFormat\0", "(Landroid/content/Context;)Ljava/text/DateFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDateFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#getDateFormat(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-text-DateFormat"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-text-DateFormat")))]
        pub fn getDateFormat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::DateFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getDateFormat", .descriptor == "(Landroid/content/Context;)Ljava/text/DateFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getDateFormat\0", "(Landroid/content/Context;)Ljava/text/DateFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLongDateFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#getLongDateFormat(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-text-DateFormat"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-text-DateFormat")))]
        pub fn getLongDateFormat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::DateFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getLongDateFormat", .descriptor == "(Landroid/content/Context;)Ljava/text/DateFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getLongDateFormat\0", "(Landroid/content/Context;)Ljava/text/DateFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMediumDateFormat](https://developer.android.com/reference/android/text/format/DateFormat.html#getMediumDateFormat(android.content.Context))
        ///
        /// Required features: "android-content-Context", "java-text-DateFormat"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-text-DateFormat")))]
        pub fn getMediumDateFormat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::text::DateFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getMediumDateFormat", .descriptor == "(Landroid/content/Context;)Ljava/text/DateFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getMediumDateFormat\0", "(Landroid/content/Context;)Ljava/text/DateFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDateFormatOrder](https://developer.android.com/reference/android/text/format/DateFormat.html#getDateFormatOrder(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn getDateFormatOrder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "getDateFormatOrder", .descriptor == "(Landroid/content/Context;)[C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "getDateFormatOrder\0", "(Landroid/content/Context;)[C\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/text/format/DateFormat.html#format(java.lang.CharSequence,%20long))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn format_CharSequence_long<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "format", .descriptor == "(Ljava/lang/CharSequence;J)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "format\0", "(Ljava/lang/CharSequence;J)Ljava/lang/CharSequence;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/text/format/DateFormat.html#format(java.lang.CharSequence,%20java.util.Date))
        ///
        /// Required features: "java-lang-CharSequence", "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-util-Date")))]
        pub fn format_CharSequence_Date<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "format", .descriptor == "(Ljava/lang/CharSequence;Ljava/util/Date;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "format\0", "(Ljava/lang/CharSequence;Ljava/util/Date;)Ljava/lang/CharSequence;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/text/format/DateFormat.html#format(java.lang.CharSequence,%20java.util.Calendar))
        ///
        /// Required features: "java-lang-CharSequence", "java-util-Calendar"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-util-Calendar")))]
        pub fn format_CharSequence_Calendar<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Calendar>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/DateFormat", java.flags == PUBLIC | STATIC, .name == "format", .descriptor == "(Ljava/lang/CharSequence;Ljava/util/Calendar;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/DateFormat\0", "format\0", "(Ljava/lang/CharSequence;Ljava/util/Calendar;)Ljava/lang/CharSequence;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
