// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
__jni_bindgen! {
    /// public enum [JsonToken](https://developer.android.com/reference/android/util/JsonToken.html)
    ///
    /// Required feature: android-util-JsonToken
    public enum JsonToken ("android/util/JsonToken") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/android/util/JsonToken.html#values())
        ///
        /// Required features: "android-util-JsonToken"
        #[cfg(any(feature = "all", all(feature = "android-util-JsonToken")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::util::JsonToken, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/JsonToken", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Landroid/util/JsonToken;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/JsonToken\0", "values\0", "()[Landroid/util/JsonToken;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/android/util/JsonToken.html#valueOf(java.lang.String))
        ///
        /// Required features: "android-util-JsonToken", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-util-JsonToken", feature = "java-lang-String")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/util/JsonToken", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Landroid/util/JsonToken;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/util/JsonToken\0", "valueOf\0", "(Ljava/lang/String;)Landroid/util/JsonToken;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [JsonToken](https://developer.android.com/reference/android/util/JsonToken.html#JsonToken(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::util::JsonToken>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/util/JsonToken", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/util/JsonToken\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [BEGIN_ARRAY](https://developer.android.com/reference/android/util/JsonToken.html#BEGIN_ARRAY)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn BEGIN_ARRAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "BEGIN_ARRAY\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [END_ARRAY](https://developer.android.com/reference/android/util/JsonToken.html#END_ARRAY)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn END_ARRAY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "END_ARRAY\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BEGIN_OBJECT](https://developer.android.com/reference/android/util/JsonToken.html#BEGIN_OBJECT)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn BEGIN_OBJECT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "BEGIN_OBJECT\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [END_OBJECT](https://developer.android.com/reference/android/util/JsonToken.html#END_OBJECT)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn END_OBJECT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "END_OBJECT\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NAME](https://developer.android.com/reference/android/util/JsonToken.html#NAME)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn NAME<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "NAME\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [STRING](https://developer.android.com/reference/android/util/JsonToken.html#STRING)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn STRING<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "STRING\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NUMBER](https://developer.android.com/reference/android/util/JsonToken.html#NUMBER)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn NUMBER<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "NUMBER\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [BOOLEAN](https://developer.android.com/reference/android/util/JsonToken.html#BOOLEAN)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn BOOLEAN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "BOOLEAN\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [NULL](https://developer.android.com/reference/android/util/JsonToken.html#NULL)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn NULL<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "NULL\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [END_DOCUMENT](https://developer.android.com/reference/android/util/JsonToken.html#END_DOCUMENT)
        ///
        /// Required feature: android-util-JsonToken
        #[cfg(any(feature = "all", feature = "android-util-JsonToken"))]
        pub fn END_DOCUMENT<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::util::JsonToken>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/util/JsonToken\0", "END_DOCUMENT\0", "Landroid/util/JsonToken;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::util::JsonToken, crate::java::lang::Throwable>>> { ... }
    }
}
