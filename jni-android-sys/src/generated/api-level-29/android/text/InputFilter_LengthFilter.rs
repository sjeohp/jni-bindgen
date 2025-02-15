// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-text-InputFilter_LengthFilter"))]
__jni_bindgen! {
    /// public class [InputFilter.LengthFilter](https://developer.android.com/reference/android/text/InputFilter.LengthFilter.html)
    ///
    /// Required feature: android-text-InputFilter_LengthFilter
    public class InputFilter_LengthFilter ("android/text/InputFilter$LengthFilter") extends crate::java::lang::Object, implements crate::android::text::InputFilter {

        /// [LengthFilter](https://developer.android.com/reference/android/text/InputFilter.LengthFilter.html#LengthFilter(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::text::InputFilter_LengthFilter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/InputFilter$LengthFilter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/InputFilter$LengthFilter\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [filter](https://developer.android.com/reference/android/text/InputFilter.LengthFilter.html#filter(java.lang.CharSequence,%20int,%20int,%20android.text.Spanned,%20int,%20int))
        ///
        /// Required features: "android-text-Spanned", "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "android-text-Spanned", feature = "java-lang-CharSequence")))]
        pub fn filter<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>, arg1: i32, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::text::Spanned>>, arg4: i32, arg5: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/InputFilter$LengthFilter", java.flags == PUBLIC, .name == "filter", .descriptor == "(Ljava/lang/CharSequence;IILandroid/text/Spanned;II)Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/InputFilter$LengthFilter\0", "filter\0", "(Ljava/lang/CharSequence;IILandroid/text/Spanned;II)Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMax](https://developer.android.com/reference/android/text/InputFilter.LengthFilter.html#getMax())
        pub fn getMax<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/text/InputFilter$LengthFilter", java.flags == PUBLIC, .name == "getMax", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/text/InputFilter$LengthFilter\0", "getMax\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
