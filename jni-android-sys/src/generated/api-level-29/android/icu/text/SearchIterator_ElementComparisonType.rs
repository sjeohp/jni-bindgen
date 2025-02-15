// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-SearchIterator_ElementComparisonType"))]
__jni_bindgen! {
    /// public enum [SearchIterator.ElementComparisonType](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html)
    ///
    /// Required feature: android-icu-text-SearchIterator_ElementComparisonType
    public enum SearchIterator_ElementComparisonType ("android/icu/text/SearchIterator$ElementComparisonType") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#values())
        ///
        /// Required features: "android-icu-text-SearchIterator_ElementComparisonType"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-SearchIterator_ElementComparisonType")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::SearchIterator_ElementComparisonType, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator$ElementComparisonType", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/SearchIterator$ElementComparisonType;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/SearchIterator$ElementComparisonType\0", "values\0", "()[Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-SearchIterator_ElementComparisonType", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-SearchIterator_ElementComparisonType", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/SearchIterator$ElementComparisonType", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/SearchIterator$ElementComparisonType;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/SearchIterator$ElementComparisonType\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [ElementComparisonType](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#ElementComparisonType(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/SearchIterator$ElementComparisonType", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/SearchIterator$ElementComparisonType\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [STANDARD_ELEMENT_COMPARISON](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#STANDARD_ELEMENT_COMPARISON)
        ///
        /// Required feature: android-icu-text-SearchIterator_ElementComparisonType
        #[cfg(any(feature = "all", feature = "android-icu-text-SearchIterator_ElementComparisonType"))]
        pub fn STANDARD_ELEMENT_COMPARISON<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/SearchIterator$ElementComparisonType\0", "STANDARD_ELEMENT_COMPARISON\0", "Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PATTERN_BASE_WEIGHT_IS_WILDCARD](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#PATTERN_BASE_WEIGHT_IS_WILDCARD)
        ///
        /// Required feature: android-icu-text-SearchIterator_ElementComparisonType
        #[cfg(any(feature = "all", feature = "android-icu-text-SearchIterator_ElementComparisonType"))]
        pub fn PATTERN_BASE_WEIGHT_IS_WILDCARD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/SearchIterator$ElementComparisonType\0", "PATTERN_BASE_WEIGHT_IS_WILDCARD\0", "Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ANY_BASE_WEIGHT_IS_WILDCARD](https://developer.android.com/reference/android/icu/text/SearchIterator.ElementComparisonType.html#ANY_BASE_WEIGHT_IS_WILDCARD)
        ///
        /// Required feature: android-icu-text-SearchIterator_ElementComparisonType
        #[cfg(any(feature = "all", feature = "android-icu-text-SearchIterator_ElementComparisonType"))]
        pub fn ANY_BASE_WEIGHT_IS_WILDCARD<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::SearchIterator_ElementComparisonType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/SearchIterator$ElementComparisonType\0", "ANY_BASE_WEIGHT_IS_WILDCARD\0", "Landroid/icu/text/SearchIterator$ElementComparisonType;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::SearchIterator_ElementComparisonType, crate::java::lang::Throwable>>> { ... }
    }
}
