// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSetSpanner"))]
__jni_bindgen! {
    /// public class [UnicodeSetSpanner](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html)
    ///
    /// Required feature: android-icu-text-UnicodeSetSpanner
    public class UnicodeSetSpanner ("android/icu/text/UnicodeSetSpanner") extends crate::java::lang::Object {

        /// [UnicodeSetSpanner](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#UnicodeSetSpanner(android.icu.text.UnicodeSet))
        ///
        /// Required features: "android-icu-text-UnicodeSet"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSet")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSet>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSetSpanner>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/text/UnicodeSet;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "<init>\0", "(Landroid/icu/text/UnicodeSet;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUnicodeSet](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#getUnicodeSet())
        ///
        /// Required features: "android-icu-text-UnicodeSet"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSet")))]
        pub fn getUnicodeSet<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "getUnicodeSet", .descriptor == "()Landroid/icu/text/UnicodeSet;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "getUnicodeSet\0", "()Landroid/icu/text/UnicodeSet;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [countIn](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#countIn(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn countIn_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "countIn", .descriptor == "(Ljava/lang/CharSequence;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "countIn\0", "(Ljava/lang/CharSequence;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [countIn](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#countIn(java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.CountMethod))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod", feature = "java-lang-CharSequence")))]
        pub fn countIn_CharSequence_CountMethod<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_CountMethod>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "countIn", .descriptor == "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "countIn\0", "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [countIn](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#countIn(java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.CountMethod,%20android.icu.text.UnicodeSet.SpanCondition))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod", "android-icu-text-UnicodeSet_SpanCondition", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod", feature = "android-icu-text-UnicodeSet_SpanCondition", feature = "java-lang-CharSequence")))]
        pub fn countIn_CharSequence_CountMethod_SpanCondition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_CountMethod>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSet_SpanCondition>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "countIn", .descriptor == "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;Landroid/icu/text/UnicodeSet$SpanCondition;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "countIn\0", "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;Landroid/icu/text/UnicodeSet$SpanCondition;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deleteFrom](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#deleteFrom(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn deleteFrom_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "deleteFrom", .descriptor == "(Ljava/lang/CharSequence;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "deleteFrom\0", "(Ljava/lang/CharSequence;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [deleteFrom](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#deleteFrom(java.lang.CharSequence,%20android.icu.text.UnicodeSet.SpanCondition))
        ///
        /// Required features: "android-icu-text-UnicodeSet_SpanCondition", "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSet_SpanCondition", feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn deleteFrom_CharSequence_SpanCondition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSet_SpanCondition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "deleteFrom", .descriptor == "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "deleteFrom\0", "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [replaceFrom](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#replaceFrom(java.lang.CharSequence,%20java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn replaceFrom_CharSequence_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "replaceFrom", .descriptor == "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "replaceFrom\0", "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [replaceFrom](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#replaceFrom(java.lang.CharSequence,%20java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.CountMethod))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod", "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod", feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn replaceFrom_CharSequence_CharSequence_CountMethod<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_CountMethod>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "replaceFrom", .descriptor == "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "replaceFrom\0", "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [replaceFrom](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#replaceFrom(java.lang.CharSequence,%20java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.CountMethod,%20android.icu.text.UnicodeSet.SpanCondition))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod", "android-icu-text-UnicodeSet_SpanCondition", "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod", feature = "android-icu-text-UnicodeSet_SpanCondition", feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn replaceFrom_CharSequence_CharSequence_CountMethod_SpanCondition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_CountMethod>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSet_SpanCondition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "replaceFrom", .descriptor == "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "replaceFrom\0", "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$CountMethod;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [trim](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#trim(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn trim_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "trim", .descriptor == "(Ljava/lang/CharSequence;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "trim\0", "(Ljava/lang/CharSequence;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [trim](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#trim(java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.TrimOption))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_TrimOption", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_TrimOption", feature = "java-lang-CharSequence")))]
        pub fn trim_CharSequence_TrimOption<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_TrimOption>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "trim", .descriptor == "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$TrimOption;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "trim\0", "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$TrimOption;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [trim](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.html#trim(java.lang.CharSequence,%20android.icu.text.UnicodeSetSpanner.TrimOption,%20android.icu.text.UnicodeSet.SpanCondition))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_TrimOption", "android-icu-text-UnicodeSet_SpanCondition", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_TrimOption", feature = "android-icu-text-UnicodeSet_SpanCondition", feature = "java-lang-CharSequence")))]
        pub fn trim_CharSequence_TrimOption_SpanCondition<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSetSpanner_TrimOption>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::UnicodeSet_SpanCondition>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner", java.flags == PUBLIC, .name == "trim", .descriptor == "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$TrimOption;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner\0", "trim\0", "(Ljava/lang/CharSequence;Landroid/icu/text/UnicodeSetSpanner$TrimOption;Landroid/icu/text/UnicodeSet$SpanCondition;)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
