// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
__jni_bindgen! {
    /// public enum [IDNA.Error](https://developer.android.com/reference/android/icu/text/IDNA.Error.html)
    ///
    /// Required feature: android-icu-text-IDNA_Error
    public enum IDNA_Error ("android/icu/text/IDNA$Error") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#values())
        ///
        /// Required features: "android-icu-text-IDNA_Error"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-IDNA_Error")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::IDNA_Error, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Error", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/IDNA$Error;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/IDNA$Error\0", "values\0", "()[Landroid/icu/text/IDNA$Error;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-IDNA_Error", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-IDNA_Error", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/IDNA$Error", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/IDNA$Error;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/IDNA$Error\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/IDNA$Error;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [Error](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#Error(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/IDNA$Error", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/IDNA$Error\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [EMPTY_LABEL](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#EMPTY_LABEL)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn EMPTY_LABEL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "EMPTY_LABEL\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LABEL_TOO_LONG](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#LABEL_TOO_LONG)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn LABEL_TOO_LONG<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "LABEL_TOO_LONG\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DOMAIN_NAME_TOO_LONG](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#DOMAIN_NAME_TOO_LONG)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn DOMAIN_NAME_TOO_LONG<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "DOMAIN_NAME_TOO_LONG\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LEADING_HYPHEN](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#LEADING_HYPHEN)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn LEADING_HYPHEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "LEADING_HYPHEN\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [TRAILING_HYPHEN](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#TRAILING_HYPHEN)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn TRAILING_HYPHEN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "TRAILING_HYPHEN\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HYPHEN_3_4](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#HYPHEN_3_4)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn HYPHEN_3_4<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "HYPHEN_3_4\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LEADING_COMBINING_MARK](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#LEADING_COMBINING_MARK)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn LEADING_COMBINING_MARK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "LEADING_COMBINING_MARK\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DISALLOWED](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#DISALLOWED)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn DISALLOWED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "DISALLOWED\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [PUNYCODE](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#PUNYCODE)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn PUNYCODE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "PUNYCODE\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [LABEL_HAS_DOT](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#LABEL_HAS_DOT)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn LABEL_HAS_DOT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "LABEL_HAS_DOT\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [INVALID_ACE_LABEL](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#INVALID_ACE_LABEL)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn INVALID_ACE_LABEL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "INVALID_ACE_LABEL\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BIDI](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#BIDI)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn BIDI<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "BIDI\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CONTEXTJ](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#CONTEXTJ)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn CONTEXTJ<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "CONTEXTJ\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CONTEXTO_PUNCTUATION](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#CONTEXTO_PUNCTUATION)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn CONTEXTO_PUNCTUATION<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "CONTEXTO_PUNCTUATION\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [CONTEXTO_DIGITS](https://developer.android.com/reference/android/icu/text/IDNA.Error.html#CONTEXTO_DIGITS)
        ///
        /// Required feature: android-icu-text-IDNA_Error
        #[cfg(any(feature = "all", feature = "android-icu-text-IDNA_Error"))]
        pub fn CONTEXTO_DIGITS<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::IDNA_Error>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/IDNA$Error\0", "CONTEXTO_DIGITS\0", "Landroid/icu/text/IDNA$Error;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::IDNA_Error, crate::java::lang::Throwable>>> { ... }
    }
}
