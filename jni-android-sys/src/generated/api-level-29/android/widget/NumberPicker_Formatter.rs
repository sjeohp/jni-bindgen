// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-NumberPicker_Formatter"))]
__jni_bindgen! {
    /// public interface [NumberPicker.Formatter](https://developer.android.com/reference/android/widget/NumberPicker.Formatter.html)
    ///
    /// Required feature: android-widget-NumberPicker_Formatter
    public interface NumberPicker_Formatter ("android/widget/NumberPicker$Formatter") extends crate::java::lang::Object {

        /// [format](https://developer.android.com/reference/android/widget/NumberPicker.Formatter.html#format(int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn format<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/NumberPicker$Formatter", java.flags == PUBLIC | ABSTRACT, .name == "format", .descriptor == "(I)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/NumberPicker$Formatter\0", "format\0", "(I)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
