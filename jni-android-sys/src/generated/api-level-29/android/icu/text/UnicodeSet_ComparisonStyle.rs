// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSet_ComparisonStyle"))]
__jni_bindgen! {
    /// public enum [UnicodeSet.ComparisonStyle](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html)
    ///
    /// Required feature: android-icu-text-UnicodeSet_ComparisonStyle
    public enum UnicodeSet_ComparisonStyle ("android/icu/text/UnicodeSet$ComparisonStyle") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#values())
        ///
        /// Required features: "android-icu-text-UnicodeSet_ComparisonStyle"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSet_ComparisonStyle")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::UnicodeSet_ComparisonStyle, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSet$ComparisonStyle", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/UnicodeSet$ComparisonStyle;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/UnicodeSet$ComparisonStyle\0", "values\0", "()[Landroid/icu/text/UnicodeSet$ComparisonStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-UnicodeSet_ComparisonStyle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-UnicodeSet_ComparisonStyle", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet_ComparisonStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/UnicodeSet$ComparisonStyle", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/UnicodeSet$ComparisonStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/UnicodeSet$ComparisonStyle\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/UnicodeSet$ComparisonStyle;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [ComparisonStyle](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#ComparisonStyle(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet_ComparisonStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/UnicodeSet$ComparisonStyle", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/UnicodeSet$ComparisonStyle\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [SHORTER_FIRST](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#SHORTER_FIRST)
        ///
        /// Required feature: android-icu-text-UnicodeSet_ComparisonStyle
        #[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSet_ComparisonStyle"))]
        pub fn SHORTER_FIRST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet_ComparisonStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/UnicodeSet$ComparisonStyle\0", "SHORTER_FIRST\0", "Landroid/icu/text/UnicodeSet$ComparisonStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LEXICOGRAPHIC](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#LEXICOGRAPHIC)
        ///
        /// Required feature: android-icu-text-UnicodeSet_ComparisonStyle
        #[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSet_ComparisonStyle"))]
        pub fn LEXICOGRAPHIC<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet_ComparisonStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/UnicodeSet$ComparisonStyle\0", "LEXICOGRAPHIC\0", "Landroid/icu/text/UnicodeSet$ComparisonStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LONGER_FIRST](https://developer.android.com/reference/android/icu/text/UnicodeSet.ComparisonStyle.html#LONGER_FIRST)
        ///
        /// Required feature: android-icu-text-UnicodeSet_ComparisonStyle
        #[cfg(any(feature = "all", feature = "android-icu-text-UnicodeSet_ComparisonStyle"))]
        pub fn LONGER_FIRST<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::UnicodeSet_ComparisonStyle>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/UnicodeSet$ComparisonStyle\0", "LONGER_FIRST\0", "Landroid/icu/text/UnicodeSet$ComparisonStyle;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::UnicodeSet_ComparisonStyle, crate::java::lang::Throwable>>> { ... }
    }
}
