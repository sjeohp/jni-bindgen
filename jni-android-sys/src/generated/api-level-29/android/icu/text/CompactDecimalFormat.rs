// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-CompactDecimalFormat"))]
__jni_bindgen! {
    /// public class [CompactDecimalFormat](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html)
    ///
    /// Required feature: android-icu-text-CompactDecimalFormat
    public class CompactDecimalFormat ("android/icu/text/CompactDecimalFormat") extends crate::android::icu::text::DecimalFormat {

        // // Not emitting: Non-public method
        // /// [CompactDecimalFormat](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html#CompactDecimalFormat(android.icu.util.ULocale,%20android.icu.text.CompactDecimalFormat.CompactStyle))
        // ///
        // /// Required features: "android-icu-text-CompactDecimalFormat_CompactStyle", "android-icu-util-ULocale"
        // #[cfg(any(feature = "all", all(feature = "android-icu-text-CompactDecimalFormat_CompactStyle", feature = "android-icu-util-ULocale")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::CompactDecimalFormat_CompactStyle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::CompactDecimalFormat>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/CompactDecimalFormat", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/icu/util/ULocale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CompactDecimalFormat\0", "<init>\0", "(Landroid/icu/util/ULocale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html#getInstance(android.icu.util.ULocale,%20android.icu.text.CompactDecimalFormat.CompactStyle))
        ///
        /// Required features: "android-icu-text-CompactDecimalFormat", "android-icu-text-CompactDecimalFormat_CompactStyle", "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-CompactDecimalFormat", feature = "android-icu-text-CompactDecimalFormat_CompactStyle", feature = "android-icu-util-ULocale")))]
        pub fn getInstance_ULocale_CompactStyle<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::CompactDecimalFormat_CompactStyle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::CompactDecimalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CompactDecimalFormat", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Landroid/icu/util/ULocale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)Landroid/icu/text/CompactDecimalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/CompactDecimalFormat\0", "getInstance\0", "(Landroid/icu/util/ULocale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)Landroid/icu/text/CompactDecimalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html#getInstance(java.util.Locale,%20android.icu.text.CompactDecimalFormat.CompactStyle))
        ///
        /// Required features: "android-icu-text-CompactDecimalFormat", "android-icu-text-CompactDecimalFormat_CompactStyle", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-CompactDecimalFormat", feature = "android-icu-text-CompactDecimalFormat_CompactStyle", feature = "java-util-Locale")))]
        pub fn getInstance_Locale_CompactStyle<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::CompactDecimalFormat_CompactStyle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::CompactDecimalFormat>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CompactDecimalFormat", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/util/Locale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)Landroid/icu/text/CompactDecimalFormat;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/CompactDecimalFormat\0", "getInstance\0", "(Ljava/util/Locale;Landroid/icu/text/CompactDecimalFormat$CompactStyle;)Landroid/icu/text/CompactDecimalFormat;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parse](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html#parse(java.lang.String,%20java.text.ParsePosition))
        ///
        /// Required features: "java-lang-Number", "java-lang-String", "java-text-ParsePosition"
        #[cfg(any(feature = "all", all(feature = "java-lang-Number", feature = "java-lang-String", feature = "java-text-ParsePosition")))]
        pub fn parse<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::ParsePosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Number>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CompactDecimalFormat", java.flags == PUBLIC, .name == "parse", .descriptor == "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CompactDecimalFormat\0", "parse\0", "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseCurrency](https://developer.android.com/reference/android/icu/text/CompactDecimalFormat.html#parseCurrency(java.lang.CharSequence,%20java.text.ParsePosition))
        ///
        /// Required features: "android-icu-util-CurrencyAmount", "java-lang-CharSequence", "java-text-ParsePosition"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-CurrencyAmount", feature = "java-lang-CharSequence", feature = "java-text-ParsePosition")))]
        pub fn parseCurrency<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::text::ParsePosition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::CurrencyAmount>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CompactDecimalFormat", java.flags == PUBLIC, .name == "parseCurrency", .descriptor == "(Ljava/lang/CharSequence;Ljava/text/ParsePosition;)Landroid/icu/util/CurrencyAmount;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CompactDecimalFormat\0", "parseCurrency\0", "(Ljava/lang/CharSequence;Ljava/text/ParsePosition;)Landroid/icu/util/CurrencyAmount;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
