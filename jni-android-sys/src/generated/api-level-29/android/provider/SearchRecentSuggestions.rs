// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-provider-SearchRecentSuggestions"))]
__jni_bindgen! {
    /// public class [SearchRecentSuggestions](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html)
    ///
    /// Required feature: android-provider-SearchRecentSuggestions
    public class SearchRecentSuggestions ("android/provider/SearchRecentSuggestions") extends crate::java::lang::Object {

        /// [SearchRecentSuggestions](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#SearchRecentSuggestions(android.content.Context,%20java.lang.String,%20int))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::provider::SearchRecentSuggestions>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/SearchRecentSuggestions", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/SearchRecentSuggestions\0", "<init>\0", "(Landroid/content/Context;Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [saveRecentQuery](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#saveRecentQuery(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn saveRecentQuery<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/SearchRecentSuggestions", java.flags == PUBLIC, .name == "saveRecentQuery", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/SearchRecentSuggestions\0", "saveRecentQuery\0", "(Ljava/lang/String;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clearHistory](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#clearHistory())
        pub fn clearHistory<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/provider/SearchRecentSuggestions", java.flags == PUBLIC, .name == "clearHistory", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/SearchRecentSuggestions\0", "clearHistory\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [truncateHistory](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#truncateHistory(android.content.ContentResolver,%20int))
        // ///
        // /// Required features: "android-content-ContentResolver"
        // #[cfg(any(feature = "all", all(feature = "android-content-ContentResolver")))]
        // fn truncateHistory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ContentResolver>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/provider/SearchRecentSuggestions", java.flags == PROTECTED, .name == "truncateHistory", .descriptor == "(Landroid/content/ContentResolver;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/provider/SearchRecentSuggestions\0", "truncateHistory\0", "(Landroid/content/ContentResolver;I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [QUERIES_PROJECTION_1LINE](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_1LINE)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn QUERIES_PROJECTION_1LINE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/SearchRecentSuggestions\0", "QUERIES_PROJECTION_1LINE\0", "[Ljava/lang/String;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [QUERIES_PROJECTION_2LINE](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_2LINE)
        ///
        /// Required feature: java-lang-String
        #[cfg(any(feature = "all", feature = "java-lang-String"))]
        pub fn QUERIES_PROJECTION_2LINE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/provider/SearchRecentSuggestions\0", "QUERIES_PROJECTION_2LINE\0", "[Ljava/lang/String;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [QUERIES_PROJECTION_DATE_INDEX](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_DATE_INDEX)
        pub const QUERIES_PROJECTION_DATE_INDEX : i32 = 1;

        /// public static final [QUERIES_PROJECTION_DISPLAY1_INDEX](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_DISPLAY1_INDEX)
        pub const QUERIES_PROJECTION_DISPLAY1_INDEX : i32 = 3;

        /// public static final [QUERIES_PROJECTION_DISPLAY2_INDEX](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_DISPLAY2_INDEX)
        pub const QUERIES_PROJECTION_DISPLAY2_INDEX : i32 = 4;

        /// public static final [QUERIES_PROJECTION_QUERY_INDEX](https://developer.android.com/reference/android/provider/SearchRecentSuggestions.html#QUERIES_PROJECTION_QUERY_INDEX)
        pub const QUERIES_PROJECTION_QUERY_INDEX : i32 = 2;
    }
}
