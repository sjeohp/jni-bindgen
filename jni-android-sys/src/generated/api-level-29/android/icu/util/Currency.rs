// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-util-Currency"))]
__jni_bindgen! {
    /// public class [Currency](https://developer.android.com/reference/android/icu/util/Currency.html)
    ///
    /// Required feature: android-icu-util-Currency
    public class Currency ("android/icu/util/Currency") extends crate::android::icu::util::MeasureUnit {

        // // Not emitting: Non-public method
        // /// [Currency](https://developer.android.com/reference/android/icu/util/Currency.html#Currency(java.lang.String))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::util::Currency>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/util/Currency", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "<init>\0", "(Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getInstance](https://developer.android.com/reference/android/icu/util/Currency.html#getInstance(java.util.Locale))
        ///
        /// Required features: "android-icu-util-Currency", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency", feature = "java-util-Locale")))]
        pub fn getInstance_Locale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::Currency>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/util/Locale;)Landroid/icu/util/Currency;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getInstance\0", "(Ljava/util/Locale;)Landroid/icu/util/Currency;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/util/Currency.html#getInstance(android.icu.util.ULocale))
        ///
        /// Required features: "android-icu-util-Currency", "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency", feature = "android-icu-util-ULocale")))]
        pub fn getInstance_ULocale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::Currency>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Landroid/icu/util/ULocale;)Landroid/icu/util/Currency;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getInstance\0", "(Landroid/icu/util/ULocale;)Landroid/icu/util/Currency;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAvailableCurrencyCodes](https://developer.android.com/reference/android/icu/util/Currency.html#getAvailableCurrencyCodes(android.icu.util.ULocale,%20java.util.Date))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String", "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String", feature = "java-util-Date")))]
        pub fn getAvailableCurrencyCodes_ULocale_Date<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getAvailableCurrencyCodes", .descriptor == "(Landroid/icu/util/ULocale;Ljava/util/Date;)[Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getAvailableCurrencyCodes\0", "(Landroid/icu/util/ULocale;Ljava/util/Date;)[Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAvailableCurrencyCodes](https://developer.android.com/reference/android/icu/util/Currency.html#getAvailableCurrencyCodes(java.util.Locale,%20java.util.Date))
        ///
        /// Required features: "java-lang-String", "java-util-Date", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Date", feature = "java-util-Locale")))]
        pub fn getAvailableCurrencyCodes_Locale_Date<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getAvailableCurrencyCodes", .descriptor == "(Ljava/util/Locale;Ljava/util/Date;)[Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getAvailableCurrencyCodes\0", "(Ljava/util/Locale;Ljava/util/Date;)[Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAvailableCurrencies](https://developer.android.com/reference/android/icu/util/Currency.html#getAvailableCurrencies())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getAvailableCurrencies<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getAvailableCurrencies", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getAvailableCurrencies\0", "()Ljava/util/Set;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/android/icu/util/Currency.html#getInstance(java.lang.String))
        ///
        /// Required features: "android-icu-util-Currency", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency", feature = "java-lang-String")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::Currency>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Landroid/icu/util/Currency;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getInstance\0", "(Ljava/lang/String;)Landroid/icu/util/Currency;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [fromJavaCurrency](https://developer.android.com/reference/android/icu/util/Currency.html#fromJavaCurrency(java.util.Currency))
        ///
        /// Required features: "android-icu-util-Currency", "java-util-Currency"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency", feature = "java-util-Currency")))]
        pub fn fromJavaCurrency<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Currency>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::Currency>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "fromJavaCurrency", .descriptor == "(Ljava/util/Currency;)Landroid/icu/util/Currency;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "fromJavaCurrency\0", "(Ljava/util/Currency;)Landroid/icu/util/Currency;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toJavaCurrency](https://developer.android.com/reference/android/icu/util/Currency.html#toJavaCurrency())
        ///
        /// Required features: "java-util-Currency"
        #[cfg(any(feature = "all", all(feature = "java-util-Currency")))]
        pub fn toJavaCurrency<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Currency>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "toJavaCurrency", .descriptor == "()Ljava/util/Currency;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "toJavaCurrency\0", "()Ljava/util/Currency;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAvailableLocales](https://developer.android.com/reference/android/icu/util/Currency.html#getAvailableLocales())
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn getAvailableLocales<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::util::Locale, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getAvailableLocales", .descriptor == "()[Ljava/util/Locale;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getAvailableLocales\0", "()[Ljava/util/Locale;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAvailableULocales](https://developer.android.com/reference/android/icu/util/Currency.html#getAvailableULocales())
        ///
        /// Required features: "android-icu-util-ULocale"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale")))]
        pub fn getAvailableULocales<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::util::ULocale, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "getAvailableULocales", .descriptor == "()[Landroid/icu/util/ULocale;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getAvailableULocales\0", "()[Landroid/icu/util/ULocale;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeywordValuesForLocale](https://developer.android.com/reference/android/icu/util/Currency.html#getKeywordValuesForLocale(java.lang.String,%20android.icu.util.ULocale,%20boolean))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getKeywordValuesForLocale<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC | FINAL, .name == "getKeywordValuesForLocale", .descriptor == "(Ljava/lang/String;Landroid/icu/util/ULocale;Z)[Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "getKeywordValuesForLocale\0", "(Ljava/lang/String;Landroid/icu/util/ULocale;Z)[Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrencyCode](https://developer.android.com/reference/android/icu/util/Currency.html#getCurrencyCode())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getCurrencyCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getCurrencyCode", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getCurrencyCode\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNumericCode](https://developer.android.com/reference/android/icu/util/Currency.html#getNumericCode())
        pub fn getNumericCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getNumericCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getNumericCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSymbol](https://developer.android.com/reference/android/icu/util/Currency.html#getSymbol())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSymbol<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getSymbol", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getSymbol\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSymbol](https://developer.android.com/reference/android/icu/util/Currency.html#getSymbol(java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getSymbol_Locale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getSymbol", .descriptor == "(Ljava/util/Locale;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getSymbol\0", "(Ljava/util/Locale;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSymbol](https://developer.android.com/reference/android/icu/util/Currency.html#getSymbol(android.icu.util.ULocale))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getSymbol_ULocale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getSymbol", .descriptor == "(Landroid/icu/util/ULocale;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getSymbol\0", "(Landroid/icu/util/ULocale;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/icu/util/Currency.html#getName(java.util.Locale,%20int,%20bool%5B%5D))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getName_Locale_int_boolean_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::BooleanArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getName", .descriptor == "(Ljava/util/Locale;I[Z)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getName\0", "(Ljava/util/Locale;I[Z)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/icu/util/Currency.html#getName(android.icu.util.ULocale,%20int,%20bool%5B%5D))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getName_ULocale_int_boolean_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::BooleanArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getName", .descriptor == "(Landroid/icu/util/ULocale;I[Z)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getName\0", "(Landroid/icu/util/ULocale;I[Z)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/icu/util/Currency.html#getName(java.util.Locale,%20int,%20java.lang.String,%20bool%5B%5D))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getName_Locale_int_String_boolean_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::BooleanArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getName", .descriptor == "(Ljava/util/Locale;ILjava/lang/String;[Z)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getName\0", "(Ljava/util/Locale;ILjava/lang/String;[Z)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/android/icu/util/Currency.html#getName(android.icu.util.ULocale,%20int,%20java.lang.String,%20bool%5B%5D))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn getName_ULocale_int_String_boolean_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::BooleanArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getName", .descriptor == "(Landroid/icu/util/ULocale;ILjava/lang/String;[Z)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getName\0", "(Landroid/icu/util/ULocale;ILjava/lang/String;[Z)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisplayName](https://developer.android.com/reference/android/icu/util/Currency.html#getDisplayName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDisplayName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getDisplayName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getDisplayName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisplayName](https://developer.android.com/reference/android/icu/util/Currency.html#getDisplayName(java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn getDisplayName_Locale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getDisplayName", .descriptor == "(Ljava/util/Locale;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getDisplayName\0", "(Ljava/util/Locale;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultFractionDigits](https://developer.android.com/reference/android/icu/util/Currency.html#getDefaultFractionDigits())
        pub fn getDefaultFractionDigits<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getDefaultFractionDigits", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getDefaultFractionDigits\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultFractionDigits](https://developer.android.com/reference/android/icu/util/Currency.html#getDefaultFractionDigits(android.icu.util.Currency.CurrencyUsage))
        ///
        /// Required features: "android-icu-util-Currency_CurrencyUsage"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency_CurrencyUsage")))]
        pub fn getDefaultFractionDigits_CurrencyUsage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::Currency_CurrencyUsage>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getDefaultFractionDigits", .descriptor == "(Landroid/icu/util/Currency$CurrencyUsage;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getDefaultFractionDigits\0", "(Landroid/icu/util/Currency$CurrencyUsage;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRoundingIncrement](https://developer.android.com/reference/android/icu/util/Currency.html#getRoundingIncrement())
        pub fn getRoundingIncrement<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getRoundingIncrement", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getRoundingIncrement\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRoundingIncrement](https://developer.android.com/reference/android/icu/util/Currency.html#getRoundingIncrement(android.icu.util.Currency.CurrencyUsage))
        ///
        /// Required features: "android-icu-util-Currency_CurrencyUsage"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-Currency_CurrencyUsage")))]
        pub fn getRoundingIncrement_CurrencyUsage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::Currency_CurrencyUsage>>) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "getRoundingIncrement", .descriptor == "(Landroid/icu/util/Currency$CurrencyUsage;)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "getRoundingIncrement\0", "(Landroid/icu/util/Currency$CurrencyUsage;)D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/icu/util/Currency.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/util/Currency\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAvailable](https://developer.android.com/reference/android/icu/util/Currency.html#isAvailable(java.lang.String,%20java.util.Date,%20java.util.Date))
        ///
        /// Required features: "java-lang-String", "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Date")))]
        pub fn isAvailable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/util/Currency", java.flags == PUBLIC | STATIC, .name == "isAvailable", .descriptor == "(Ljava/lang/String;Ljava/util/Date;Ljava/util/Date;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/util/Currency\0", "isAvailable\0", "(Ljava/lang/String;Ljava/util/Date;Ljava/util/Date;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [LONG_NAME](https://developer.android.com/reference/android/icu/util/Currency.html#LONG_NAME)
        pub const LONG_NAME : i32 = 1;

        /// public static final [PLURAL_LONG_NAME](https://developer.android.com/reference/android/icu/util/Currency.html#PLURAL_LONG_NAME)
        pub const PLURAL_LONG_NAME : i32 = 2;

        /// public static final [SYMBOL_NAME](https://developer.android.com/reference/android/icu/util/Currency.html#SYMBOL_NAME)
        pub const SYMBOL_NAME : i32 = 0;
    }
}
