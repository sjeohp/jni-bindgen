// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSetSpanner_CountMethod"))]
__jni_bindgen! {
    /// public enum [UnicodeSetSpanner.CountMethod](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html)
    ///
    /// Required feature: android-icu-text-UnicodeSetSpanner_CountMethod
    public enum UnicodeSetSpanner_CountMethod ("android/icu/text/UnicodeSetSpanner$CountMethod") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html#values())
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::UnicodeSetSpanner_CountMethod, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner$CountMethod", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/UnicodeSetSpanner$CountMethod;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/UnicodeSetSpanner$CountMethod\0", "values\0", "()[Landroid/icu/text/UnicodeSetSpanner$CountMethod;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-UnicodeSetSpanner_CountMethod", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSetSpanner_CountMethod", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSetSpanner_CountMethod>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSetSpanner$CountMethod", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/UnicodeSetSpanner$CountMethod;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/UnicodeSetSpanner$CountMethod\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/UnicodeSetSpanner$CountMethod;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [CountMethod](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html#CountMethod(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSetSpanner_CountMethod>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/UnicodeSetSpanner$CountMethod", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSetSpanner$CountMethod\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [WHOLE_SPAN](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html#WHOLE_SPAN)
        ///
        /// Required feature: android-icu-text-UnicodeSetSpanner_CountMethod
        #[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSetSpanner_CountMethod"))]
        pub fn WHOLE_SPAN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSetSpanner_CountMethod>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/UnicodeSetSpanner$CountMethod\0", "WHOLE_SPAN\0", "Landroid/icu/text/UnicodeSetSpanner$CountMethod;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MIN_ELEMENTS](https://developer.android.com/reference/android/icu/text/UnicodeSetSpanner.CountMethod.html#MIN_ELEMENTS)
        ///
        /// Required feature: android-icu-text-UnicodeSetSpanner_CountMethod
        #[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSetSpanner_CountMethod"))]
        pub fn MIN_ELEMENTS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSetSpanner_CountMethod>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/UnicodeSetSpanner$CountMethod\0", "MIN_ELEMENTS\0", "Landroid/icu/text/UnicodeSetSpanner$CountMethod;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::UnicodeSetSpanner_CountMethod, crate::java::lang::Throwable>>> { ... }
    }
}
