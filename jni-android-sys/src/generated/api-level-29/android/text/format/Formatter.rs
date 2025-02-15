// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-format-Formatter"))]
__jni_bindgen! {
    /// public final class [Formatter](https://developer.android.com/reference/android/text/format/Formatter.html)
    ///
    /// Required feature: android-text-format-Formatter
    public final class Formatter ("android/text/format/Formatter") extends crate::java::lang::Object {

        /// [Formatter](https://developer.android.com/reference/android/text/format/Formatter.html#Formatter())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::format::Formatter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/Formatter", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/format/Formatter\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [formatFileSize](https://developer.android.com/reference/android/text/format/Formatter.html#formatFileSize(android.content.Context,%20long))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn formatFileSize<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/Formatter", java.flags == PUBLIC | STATIC, .name == "formatFileSize", .descriptor == "(Landroid/content/Context;J)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/Formatter\0", "formatFileSize\0", "(Landroid/content/Context;J)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [formatShortFileSize](https://developer.android.com/reference/android/text/format/Formatter.html#formatShortFileSize(android.content.Context,%20long))
        ///
        /// Required features: "android-content-Context", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "java-lang-String")))]
        pub fn formatShortFileSize<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/Formatter", java.flags == PUBLIC | STATIC, .name == "formatShortFileSize", .descriptor == "(Landroid/content/Context;J)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/Formatter\0", "formatShortFileSize\0", "(Landroid/content/Context;J)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [formatIpAddress](https://developer.android.com/reference/android/text/format/Formatter.html#formatIpAddress(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn formatIpAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/format/Formatter", java.flags == PUBLIC | STATIC, .name == "formatIpAddress", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/text/format/Formatter\0", "formatIpAddress\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
