// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-TtsSpan_Builder"))]
__jni_bindgen! {
    /// public class [TtsSpan.Builder](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html)
    ///
    /// Required feature: android-text-style-TtsSpan_Builder
    public class TtsSpan_Builder ("android/text/style/TtsSpan$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html#Builder(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$Builder\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html#build())
        ///
        /// Required features: "android-text-style-TtsSpan"
        #[cfg(any(feature = "all", all(feature = "android-text-style-TtsSpan")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/text/style/TtsSpan;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$Builder\0", "build\0", "()Landroid/text/style/TtsSpan;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setStringArgument](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html#setStringArgument(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "android-text-style-TtsSpan_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-text-style-TtsSpan_Builder", feature = "java-lang-String")))]
        pub fn setStringArgument<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$Builder", java.flags == PUBLIC, .name == "setStringArgument", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Landroid/text/style/TtsSpan$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$Builder\0", "setStringArgument\0", "(Ljava/lang/String;Ljava/lang/String;)Landroid/text/style/TtsSpan$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setIntArgument](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html#setIntArgument(java.lang.String,%20int))
        ///
        /// Required features: "android-text-style-TtsSpan_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-text-style-TtsSpan_Builder", feature = "java-lang-String")))]
        pub fn setIntArgument<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$Builder", java.flags == PUBLIC, .name == "setIntArgument", .descriptor == "(Ljava/lang/String;I)Landroid/text/style/TtsSpan$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$Builder\0", "setIntArgument\0", "(Ljava/lang/String;I)Landroid/text/style/TtsSpan$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLongArgument](https://developer.android.com/reference/android/text/style/TtsSpan.Builder.html#setLongArgument(java.lang.String,%20long))
        ///
        /// Required features: "android-text-style-TtsSpan_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-text-style-TtsSpan_Builder", feature = "java-lang-String")))]
        pub fn setLongArgument<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$Builder", java.flags == PUBLIC, .name == "setLongArgument", .descriptor == "(Ljava/lang/String;J)Landroid/text/style/TtsSpan$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$Builder\0", "setLongArgument\0", "(Ljava/lang/String;J)Landroid/text/style/TtsSpan$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
