// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-icu-text-LocaleDisplayNames_UiListItem"))]
__jni_bindgen! {
    /// public class [LocaleDisplayNames.UiListItem](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html)
    ///
    /// Required feature: android-icu-text-LocaleDisplayNames_UiListItem
    public class LocaleDisplayNames_UiListItem ("android/icu/text/LocaleDisplayNames$UiListItem") extends crate::java::lang::Object {

        /// [UiListItem](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#UiListItem(android.icu.util.ULocale,%20android.icu.util.ULocale,%20java.lang.String,%20java.lang.String))
        ///
        /// Required features: "android-icu-util-ULocale", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::icu::text::LocaleDisplayNames_UiListItem>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/LocaleDisplayNames$UiListItem", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/icu/util/ULocale;Landroid/icu/util/ULocale;Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/LocaleDisplayNames$UiListItem\0", "<init>\0", "(Landroid/icu/util/ULocale;Landroid/icu/util/ULocale;Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/LocaleDisplayNames$UiListItem", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/LocaleDisplayNames$UiListItem\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/LocaleDisplayNames$UiListItem", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/LocaleDisplayNames$UiListItem\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/LocaleDisplayNames$UiListItem", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/icu/text/LocaleDisplayNames$UiListItem\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getComparator](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#getComparator(java.util.Comparator,%20boolean))
        ///
        /// Required features: "java-util-Comparator"
        #[cfg(any(feature = "all", all(feature = "java-util-Comparator")))]
        pub fn getComparator<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Comparator>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Comparator>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/icu/text/LocaleDisplayNames$UiListItem", java.flags == PUBLIC | STATIC, .name == "getComparator", .descriptor == "(Ljava/util/Comparator;Z)Ljava/util/Comparator;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/icu/text/LocaleDisplayNames$UiListItem\0", "getComparator\0", "(Ljava/util/Comparator;Z)Ljava/util/Comparator;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public final [minimized](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#minimized)
        ///
        /// Required feature: android-icu-util-ULocale
        #[cfg(any(feature = "all", feature = "android-icu-util-ULocale"))]
        pub fn minimized<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::ULocale>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/icu/text/LocaleDisplayNames$UiListItem\0", "minimized\0", "Landroid/icu/util/ULocale;\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [modified](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#modified)
        ///
        /// Required feature: android-icu-util-ULocale
        #[cfg(any(feature = "all", feature = "android-icu-util-ULocale"))]
        pub fn modified<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::icu::util::ULocale>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/icu/text/LocaleDisplayNames$UiListItem\0", "modified\0", "Landroid/icu/util/ULocale;\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [nameInDisplayLocale](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#nameInDisplayLocale)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn nameInDisplayLocale<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/icu/text/LocaleDisplayNames$UiListItem\0", "nameInDisplayLocale\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }

        /// **get** public final [nameInSelf](https://developer.android.com/reference/android/icu/text/LocaleDisplayNames.UiListItem.html#nameInSelf)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn nameInSelf<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>> {
            unsafe {
                let env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (class, field) = env.require_class_field("android/icu/text/LocaleDisplayNames$UiListItem\0", "nameInSelf\0", "Ljava/lang/String;\0");
                env.get_object_field(class, field)
            }
        }
    }
}
