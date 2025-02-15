// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-DateIntervalFormat"))]
__jni_bindgen! {
    /// public class [DateIntervalFormat](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html)
    ///
    /// Required feature: android-icu-text-DateIntervalFormat
    public class DateIntervalFormat ("android/icu/text/DateIntervalFormat") extends crate::android::icu::text::UFormat {

        // // Not emitting: Non-public method
        // /// [DateIntervalFormat](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#DateIntervalFormat())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/DateIntervalFormat", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "java-lang-String")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String,%20java.util.Locale))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getInstance_String_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;Ljava/util/Locale;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String,%20android.icu.util.ULocale))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getInstance_String_ULocale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Landroid/icu/util/ULocale;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;Landroid/icu/util/ULocale;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String,%20android.icu.text.DateIntervalInfo))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "android-icu-text-DateIntervalInfo", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "android-icu-text-DateIntervalInfo", feature = "java-lang-String")))]
        pub fn getInstance_String_DateIntervalInfo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::DateIntervalInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String,%20java.util.Locale,%20android.icu.text.DateIntervalInfo))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "android-icu-text-DateIntervalInfo", "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "android-icu-text-DateIntervalInfo", feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getInstance_String_Locale_DateIntervalInfo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::DateIntervalInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;Ljava/util/Locale;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getInstance(java.lang.String,%20android.icu.util.ULocale,%20android.icu.text.DateIntervalInfo))
        ///
        /// Required features: "android-icu-text-DateIntervalFormat", "android-icu-text-DateIntervalInfo", "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalFormat", feature = "android-icu-text-DateIntervalInfo", feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getInstance_String_ULocale_DateIntervalInfo<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::DateIntervalInfo>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Landroid/icu/util/ULocale;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/DateIntervalFormat\0", "getInstance\0", "(Ljava/lang/String;Landroid/icu/util/ULocale;Landroid/icu/text/DateIntervalInfo;)Landroid/icu/text/DateIntervalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | SYNCRONIZED, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#format(java.lang.Object,%20java.lang.StringBuffer,%20java.text.FieldPosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-StringBuffer", "java-text-FieldPosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-StringBuffer", feature = "java-text-FieldPosition")))]
        pub fn format_Object_StringBuffer_FieldPosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::StringBuffer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::FieldPosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::StringBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | FINAL, .name == "format", .descriptor == "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "format\0", "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#format(android.icu.util.DateInterval,%20java.lang.StringBuffer,%20java.text.FieldPosition))
        ///
        /// Required features: "android-icu-util-DateInterval", "java-lang-StringBuffer", "java-text-FieldPosition"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-DateInterval", feature = "java-lang-StringBuffer", feature = "java-text-FieldPosition")))]
        pub fn format_DateInterval_StringBuffer_FieldPosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::DateInterval>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::StringBuffer>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::FieldPosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::StringBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | FINAL | SYNCRONIZED, .name == "format", .descriptor == "(Landroid/icu/util/DateInterval;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "format\0", "(Landroid/icu/util/DateInterval;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [format](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#format(android.icu.util.Calendar,%20android.icu.util.Calendar,%20java.lang.StringBuffer,%20java.text.FieldPosition))
        ///
        /// Required features: "android-icu-util-Calendar", "java-lang-StringBuffer", "java-text-FieldPosition"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Calendar", feature = "java-lang-StringBuffer", feature = "java-text-FieldPosition")))]
        pub fn format_Calendar_Calendar_StringBuffer_FieldPosition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::Calendar>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::Calendar>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::StringBuffer>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::FieldPosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::StringBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | FINAL | SYNCRONIZED, .name == "format", .descriptor == "(Landroid/icu/util/Calendar;Landroid/icu/util/Calendar;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "format\0", "(Landroid/icu/util/Calendar;Landroid/icu/util/Calendar;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseObject](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#parseObject(java.lang.String,%20java.text.ParsePosition))
        ///
        /// Required features: "java-lang-Object", "java-lang-String", "java-text-ParsePosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object", feature = "java-lang-String", feature = "java-text-ParsePosition")))]
        #[deprecated] pub fn parseObject<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::ParsePosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC, .name == "parseObject", .descriptor == "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "parseObject\0", "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDateIntervalInfo](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getDateIntervalInfo())
        ///
        /// Required features: "android-icu-text-DateIntervalInfo"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalInfo")))]
        pub fn getDateIntervalInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateIntervalInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC, .name == "getDateIntervalInfo", .descriptor == "()Landroid/icu/text/DateIntervalInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "getDateIntervalInfo\0", "()Landroid/icu/text/DateIntervalInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDateIntervalInfo](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#setDateIntervalInfo(android.icu.text.DateIntervalInfo))
        ///
        /// Required features: "android-icu-text-DateIntervalInfo"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateIntervalInfo")))]
        pub fn setDateIntervalInfo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::DateIntervalInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC, .name == "setDateIntervalInfo", .descriptor == "(Landroid/icu/text/DateIntervalInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "setDateIntervalInfo\0", "(Landroid/icu/text/DateIntervalInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeZone](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getTimeZone())
        ///
        /// Required features: "android-icu-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-TimeZone")))]
        pub fn getTimeZone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::TimeZone>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC, .name == "getTimeZone", .descriptor == "()Landroid/icu/util/TimeZone;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "getTimeZone\0", "()Landroid/icu/util/TimeZone;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeZone](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#setTimeZone(android.icu.util.TimeZone))
        ///
        /// Required features: "android-icu-util-TimeZone"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-TimeZone")))]
        pub fn setTimeZone<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::TimeZone>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC, .name == "setTimeZone", .descriptor == "(Landroid/icu/util/TimeZone;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "setTimeZone\0", "(Landroid/icu/util/TimeZone;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDateFormat](https://developer.android.com/reference/android/icu/text/DateIntervalFormat.html#getDateFormat())
        ///
        /// Required features: "android-icu-text-DateFormat"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-DateFormat")))]
        pub fn getDateFormat<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::DateFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/DateIntervalFormat", java.flags == PUBLIC | SYNCRONIZED, .name == "getDateFormat", .descriptor == "()Landroid/icu/text/DateFormat;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/DateIntervalFormat\0", "getDateFormat\0", "()Landroid/icu/text/DateFormat;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
