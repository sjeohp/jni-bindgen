// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-style-LineHeightSpan"))]
__jni_bindgen! {
    /// public interface [LineHeightSpan](https://developer.android.com/reference/android/text/style/LineHeightSpan.html)
    ///
    /// Required feature: android-text-style-LineHeightSpan
    public interface LineHeightSpan ("android/text/style/LineHeightSpan") extends crate::java::lang::Object, implements crate::android::text::style::ParagraphStyle, crate::android::text::style::WrapTogetherSpan {

        /// [chooseHeight](https://developer.android.com/reference/android/text/style/LineHeightSpan.html#chooseHeight(java.lang.CharSequence,%20int,%20int,%20int,%20int,%20android.graphics.Paint.FontMetricsInt))
        ///
        /// Required features: "android-graphics-Paint_FontMetricsInt", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Paint_FontMetricsInt", feature = "java-lang-CharSequence")))]
        pub fn chooseHeight<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Paint_FontMetricsInt>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/style/LineHeightSpan", java.flags == PUBLIC | ABSTRACT, .name == "chooseHeight", .descriptor == "(Ljava/lang/CharSequence;IIIILandroid/graphics/Paint$FontMetricsInt;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/style/LineHeightSpan\0", "chooseHeight\0", "(Ljava/lang/CharSequence;IIIILandroid/graphics/Paint$FontMetricsInt;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
