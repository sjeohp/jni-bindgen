// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-TtsSpan_VerbatimBuilder"))]
__jni_bindgen! {
    /// public class [TtsSpan.VerbatimBuilder](https://developer.android.com/reference/android/text/style/TtsSpan.VerbatimBuilder.html)
    ///
    /// Required feature: android-text-style-TtsSpan_VerbatimBuilder
    public class TtsSpan_VerbatimBuilder ("android/text/style/TtsSpan$VerbatimBuilder") extends crate::android::text::style::TtsSpan_SemioticClassBuilder {

        /// [VerbatimBuilder](https://developer.android.com/reference/android/text/style/TtsSpan.VerbatimBuilder.html#VerbatimBuilder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_VerbatimBuilder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$VerbatimBuilder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$VerbatimBuilder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [VerbatimBuilder](https://developer.android.com/reference/android/text/style/TtsSpan.VerbatimBuilder.html#VerbatimBuilder(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_VerbatimBuilder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$VerbatimBuilder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$VerbatimBuilder\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setVerbatim](https://developer.android.com/reference/android/text/style/TtsSpan.VerbatimBuilder.html#setVerbatim(java.lang.String))
        ///
        /// Required features: "android-text-style-TtsSpan_VerbatimBuilder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-text-style-TtsSpan_VerbatimBuilder", feature = "java-lang-String")))]
        pub fn setVerbatim<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::text::style::TtsSpan_VerbatimBuilder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/TtsSpan$VerbatimBuilder", java.flags == PUBLIC, .name == "setVerbatim", .descriptor == "(Ljava/lang/String;)Landroid/text/style/TtsSpan$VerbatimBuilder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/TtsSpan$VerbatimBuilder\0", "setVerbatim\0", "(Ljava/lang/String;)Landroid/text/style/TtsSpan$VerbatimBuilder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
