// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-PluralRules_PluralType"))]
__jni_bindgen! {
    /// public enum [PluralRules.PluralType](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html)
    ///
    /// Required feature: android-icu-text-PluralRules_PluralType
    public enum PluralRules_PluralType ("android/icu/text/PluralRules$PluralType") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html#values())
        ///
        /// Required features: "android-icu-text-PluralRules_PluralType"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-PluralRules_PluralType")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::PluralRules_PluralType, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/PluralRules$PluralType", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/PluralRules$PluralType;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/PluralRules$PluralType\0", "values\0", "()[Landroid/icu/text/PluralRules$PluralType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-PluralRules_PluralType", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-PluralRules_PluralType", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::PluralRules_PluralType>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/PluralRules$PluralType", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/PluralRules$PluralType;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/PluralRules$PluralType\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/PluralRules$PluralType;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [PluralType](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html#PluralType(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::PluralRules_PluralType>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/PluralRules$PluralType", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/PluralRules$PluralType\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [CARDINAL](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html#CARDINAL)
        ///
        /// Required feature: android-icu-text-PluralRules_PluralType
        #[cfg(any(feature = "all", feature = "android-icu-text-PluralRules_PluralType"))]
        pub fn CARDINAL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::PluralRules_PluralType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/PluralRules$PluralType\0", "CARDINAL\0", "Landroid/icu/text/PluralRules$PluralType;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [ORDINAL](https://developer.android.com/reference/android/icu/text/PluralRules.PluralType.html#ORDINAL)
        ///
        /// Required feature: android-icu-text-PluralRules_PluralType
        #[cfg(any(feature = "all", feature = "android-icu-text-PluralRules_PluralType"))]
        pub fn ORDINAL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::PluralRules_PluralType>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/PluralRules$PluralType\0", "ORDINAL\0", "Landroid/icu/text/PluralRules$PluralType;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::PluralRules_PluralType, crate::java::lang::Throwable>>> { ... }
    }
}
