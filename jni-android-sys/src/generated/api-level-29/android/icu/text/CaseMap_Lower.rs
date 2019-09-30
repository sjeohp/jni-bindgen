// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-CaseMap_Lower"))]
__jni_bindgen! {
    /// public final class [CaseMap.Lower](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html)
    ///
    /// Required feature: android-icu-text-CaseMap_Lower
    public final class CaseMap_Lower ("android/icu/text/CaseMap$Lower") extends crate::android::icu::text::CaseMap {

        // // Not emitting: Non-public method
        // /// [Lower](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html#Lower(int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::CaseMap_Lower>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/CaseMap$Lower", java.flags == (empty), .name == "<init>", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CaseMap$Lower\0", "<init>\0", "(I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [omitUnchangedText](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html#omitUnchangedText())
        ///
        /// Required features: "android-icu-text-CaseMap_Lower"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-CaseMap_Lower")))]
        pub fn omitUnchangedText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::CaseMap_Lower>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CaseMap$Lower", java.flags == PUBLIC, .name == "omitUnchangedText", .descriptor == "()Landroid/icu/text/CaseMap$Lower;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CaseMap$Lower\0", "omitUnchangedText\0", "()Landroid/icu/text/CaseMap$Lower;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html#apply(java.util.Locale,%20java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence", "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence", feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn apply_Locale_CharSequence<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CaseMap$Lower", java.flags == PUBLIC, .name == "apply", .descriptor == "(Ljava/util/Locale;Ljava/lang/CharSequence;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CaseMap$Lower\0", "apply\0", "(Ljava/util/Locale;Ljava/lang/CharSequence;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [apply](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html#apply(java.util.Locale,%20java.lang.CharSequence,%20java.lang.Appendable,%20android.icu.text.Edits))
        ///
        /// Required features: "android-icu-text-Edits", "java-lang-Appendable", "java-lang-CharSequence", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-Edits", feature = "java-lang-Appendable", feature = "java-lang-CharSequence", feature = "java-util-Locale")))]
        pub fn apply_Locale_CharSequence_Appendable_Edits<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Appendable>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::text::Edits>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Appendable>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/CaseMap$Lower", java.flags == PUBLIC, .name == "apply", .descriptor == "(Ljava/util/Locale;Ljava/lang/CharSequence;Ljava/lang/Appendable;Landroid/icu/text/Edits;)Ljava/lang/Appendable;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CaseMap$Lower\0", "apply\0", "(Ljava/util/Locale;Ljava/lang/CharSequence;Ljava/lang/Appendable;Landroid/icu/text/Edits;)Ljava/lang/Appendable;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [omitUnchangedText](https://developer.android.com/reference/android/icu/text/CaseMap.Lower.html#omitUnchangedText())
        // ///
        // /// Required features: "android-icu-text-CaseMap"
        // #[cfg(any(feature = "all", all(feature = "android-icu-text-CaseMap")))]
        // pub fn omitUnchangedText<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::CaseMap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/CaseMap$Lower", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "omitUnchangedText", .descriptor == "()Landroid/icu/text/CaseMap;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/CaseMap$Lower\0", "omitUnchangedText\0", "()Landroid/icu/text/CaseMap;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
