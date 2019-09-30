// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
__jni_bindgen! {
    /// public enum [RelativeDateTimeFormatter.RelativeDateTimeUnit](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html)
    ///
    /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
    public enum RelativeDateTimeFormatter_RelativeDateTimeUnit ("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#values())
        ///
        /// Required features: "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "values\0", "()[Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "valueOf\0", "(Ljava/lang/String;)Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [RelativeDateTimeUnit](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#RelativeDateTimeUnit(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [YEAR](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#YEAR)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn YEAR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "YEAR\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [QUARTER](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#QUARTER)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn QUARTER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "QUARTER\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MONTH](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#MONTH)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn MONTH<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "MONTH\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WEEK](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#WEEK)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn WEEK<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "WEEK\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [DAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#DAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn DAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "DAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [HOUR](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#HOUR)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn HOUR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "HOUR\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MINUTE](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#MINUTE)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn MINUTE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "MINUTE\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SECOND](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#SECOND)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn SECOND<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "SECOND\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SUNDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#SUNDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn SUNDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "SUNDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [MONDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#MONDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn MONDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "MONDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [TUESDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#TUESDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn TUESDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "TUESDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [WEDNESDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#WEDNESDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn WEDNESDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "WEDNESDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [THURSDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#THURSDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn THURSDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "THURSDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [FRIDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#FRIDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn FRIDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "FRIDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [SATURDAY](https://developer.android.com/reference/android/icu/text/RelativeDateTimeFormatter.RelativeDateTimeUnit.html#SATURDAY)
        ///
        /// Required feature: android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit
        #[cfg(any(feature = "all", feature = "android-icu-text-RelativeDateTimeFormatter_RelativeDateTimeUnit"))]
        pub fn SATURDAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit\0", "SATURDAY\0", "Landroid/icu/text/RelativeDateTimeFormatter$RelativeDateTimeUnit;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::icu::text::RelativeDateTimeFormatter_RelativeDateTimeUnit, crate::java::lang::Throwable>>> { ... }
    }
}
