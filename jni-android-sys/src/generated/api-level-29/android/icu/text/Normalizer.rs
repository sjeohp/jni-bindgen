// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-Normalizer"))]
__jni_bindgen! {
    /// public final class [Normalizer](https://developer.android.com/reference/android/icu/text/Normalizer.html)
    ///
    /// Required feature: android-icu-text-Normalizer
    public final class Normalizer ("android/icu/text/Normalizer") extends crate::java::lang::Object, implements crate::java::lang::Cloneable {

        // // Not emitting: Non-public method
        // /// [Normalizer](https://developer.android.com/reference/android/icu/text/Normalizer.html#Normalizer())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::Normalizer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/Normalizer", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/Normalizer\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [clone](https://developer.android.com/reference/android/icu/text/Normalizer.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        #[deprecated] pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/Normalizer\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/android/icu/text/Normalizer.html#compare(char%5B%5D,%20int,%20int,%20char%5B%5D,%20int,%20int,%20int))
        pub fn compare_char_array_int_int_char_array_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg4: i32, arg5: i32, arg6: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "([CII[CIII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/Normalizer\0", "compare\0", "([CII[CIII)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/android/icu/text/Normalizer.html#compare(java.lang.String,%20java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn compare_String_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "(Ljava/lang/String;Ljava/lang/String;I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/Normalizer\0", "compare\0", "(Ljava/lang/String;Ljava/lang/String;I)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/android/icu/text/Normalizer.html#compare(char%5B%5D,%20char%5B%5D,%20int))
        pub fn compare_char_array_char_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "([C[CI)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/Normalizer\0", "compare\0", "([C[CI)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/android/icu/text/Normalizer.html#compare(int,%20int,%20int))
        pub fn compare_int_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "(III)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/Normalizer\0", "compare\0", "(III)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/android/icu/text/Normalizer.html#compare(int,%20java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn compare_int_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/Normalizer", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "(ILjava/lang/String;I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/Normalizer\0", "compare\0", "(ILjava/lang/String;I)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [COMPARE_CODE_POINT_ORDER](https://developer.android.com/reference/android/icu/text/Normalizer.html#COMPARE_CODE_POINT_ORDER)
        pub const COMPARE_CODE_POINT_ORDER : i32 = 32768;

        /// public static final [COMPARE_IGNORE_CASE](https://developer.android.com/reference/android/icu/text/Normalizer.html#COMPARE_IGNORE_CASE)
        pub const COMPARE_IGNORE_CASE : i32 = 65536;

        /// public static final [FOLD_CASE_DEFAULT](https://developer.android.com/reference/android/icu/text/Normalizer.html#FOLD_CASE_DEFAULT)
        pub const FOLD_CASE_DEFAULT : i32 = 0;

        /// public static final [FOLD_CASE_EXCLUDE_SPECIAL_I](https://developer.android.com/reference/android/icu/text/Normalizer.html#FOLD_CASE_EXCLUDE_SPECIAL_I)
        pub const FOLD_CASE_EXCLUDE_SPECIAL_I : i32 = 1;

        /// public static final [INPUT_IS_FCD](https://developer.android.com/reference/android/icu/text/Normalizer.html#INPUT_IS_FCD)
        pub const INPUT_IS_FCD : i32 = 131072;

        /// **get** public static final [MAYBE](https://developer.android.com/reference/android/icu/text/Normalizer.html#MAYBE)
        ///
        /// Required feature: android-icu-text-Normalizer_QuickCheckResult
        #[cfg(any(feature = "all", feature = "android-icu-text-Normalizer_QuickCheckResult"))]
        pub fn MAYBE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::Normalizer_QuickCheckResult>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/Normalizer\0", "MAYBE\0", "Landroid/icu/text/Normalizer$QuickCheckResult;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NO](https://developer.android.com/reference/android/icu/text/Normalizer.html#NO)
        ///
        /// Required feature: android-icu-text-Normalizer_QuickCheckResult
        #[cfg(any(feature = "all", feature = "android-icu-text-Normalizer_QuickCheckResult"))]
        pub fn NO<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::Normalizer_QuickCheckResult>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/Normalizer\0", "NO\0", "Landroid/icu/text/Normalizer$QuickCheckResult;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [YES](https://developer.android.com/reference/android/icu/text/Normalizer.html#YES)
        ///
        /// Required feature: android-icu-text-Normalizer_QuickCheckResult
        #[cfg(any(feature = "all", feature = "android-icu-text-Normalizer_QuickCheckResult"))]
        pub fn YES<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::Normalizer_QuickCheckResult>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/Normalizer\0", "YES\0", "Landroid/icu/text/Normalizer$QuickCheckResult;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
