// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-UrlQuerySanitizer_IllegalCharacterValueSanitizer"))]
__jni_bindgen! {
    /// public class [UrlQuerySanitizer.IllegalCharacterValueSanitizer](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html)
    ///
    /// Required feature: android-net-UrlQuerySanitizer_IllegalCharacterValueSanitizer
    public class UrlQuerySanitizer_IllegalCharacterValueSanitizer ("android/net/UrlQuerySanitizer$IllegalCharacterValueSanitizer") extends crate::java::lang::Object, implements crate::android::net::UrlQuerySanitizer_ValueSanitizer {

        /// [IllegalCharacterValueSanitizer](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#IllegalCharacterValueSanitizer(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::UrlQuerySanitizer_IllegalCharacterValueSanitizer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/UrlQuerySanitizer$IllegalCharacterValueSanitizer", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/UrlQuerySanitizer$IllegalCharacterValueSanitizer\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sanitize](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#sanitize(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn sanitize<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/UrlQuerySanitizer$IllegalCharacterValueSanitizer", java.flags == PUBLIC, .name == "sanitize", .descriptor == "(Ljava/lang/String;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/UrlQuerySanitizer$IllegalCharacterValueSanitizer\0", "sanitize\0", "(Ljava/lang/String;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ALL_BUT_NUL_AND_ANGLE_BRACKETS_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_BUT_NUL_AND_ANGLE_BRACKETS_LEGAL)
        pub const ALL_BUT_NUL_AND_ANGLE_BRACKETS_LEGAL : i32 = 1439;

        /// public static final [ALL_BUT_NUL_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_BUT_NUL_LEGAL)
        pub const ALL_BUT_NUL_LEGAL : i32 = 1535;

        /// public static final [ALL_BUT_WHITESPACE_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_BUT_WHITESPACE_LEGAL)
        pub const ALL_BUT_WHITESPACE_LEGAL : i32 = 1532;

        /// public static final [ALL_ILLEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_ILLEGAL)
        pub const ALL_ILLEGAL : i32 = 0;

        /// public static final [ALL_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_OK)
        pub const ALL_OK : i32 = 2047;

        /// public static final [ALL_WHITESPACE_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#ALL_WHITESPACE_OK)
        pub const ALL_WHITESPACE_OK : i32 = 3;

        /// public static final [AMP_AND_SPACE_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#AMP_AND_SPACE_LEGAL)
        pub const AMP_AND_SPACE_LEGAL : i32 = 129;

        /// public static final [AMP_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#AMP_LEGAL)
        pub const AMP_LEGAL : i32 = 128;

        /// public static final [AMP_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#AMP_OK)
        pub const AMP_OK : i32 = 128;

        /// public static final [DQUOTE_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#DQUOTE_OK)
        pub const DQUOTE_OK : i32 = 8;

        /// public static final [GT_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#GT_OK)
        pub const GT_OK : i32 = 64;

        /// public static final [LT_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#LT_OK)
        pub const LT_OK : i32 = 32;

        /// public static final [NON_7_BIT_ASCII_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#NON_7_BIT_ASCII_OK)
        pub const NON_7_BIT_ASCII_OK : i32 = 4;

        /// public static final [NUL_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#NUL_OK)
        pub const NUL_OK : i32 = 512;

        /// public static final [OTHER_WHITESPACE_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#OTHER_WHITESPACE_OK)
        pub const OTHER_WHITESPACE_OK : i32 = 2;

        /// public static final [PCT_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#PCT_OK)
        pub const PCT_OK : i32 = 256;

        /// public static final [SCRIPT_URL_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#SCRIPT_URL_OK)
        pub const SCRIPT_URL_OK : i32 = 1024;

        /// public static final [SPACE_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#SPACE_LEGAL)
        pub const SPACE_LEGAL : i32 = 1;

        /// public static final [SPACE_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#SPACE_OK)
        pub const SPACE_OK : i32 = 1;

        /// public static final [SQUOTE_OK](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#SQUOTE_OK)
        pub const SQUOTE_OK : i32 = 16;

        /// public static final [URL_AND_SPACE_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#URL_AND_SPACE_LEGAL)
        pub const URL_AND_SPACE_LEGAL : i32 = 405;

        /// public static final [URL_LEGAL](https://developer.android.com/reference/android/net/UrlQuerySanitizer.IllegalCharacterValueSanitizer.html#URL_LEGAL)
        pub const URL_LEGAL : i32 = 404;
    }
}
