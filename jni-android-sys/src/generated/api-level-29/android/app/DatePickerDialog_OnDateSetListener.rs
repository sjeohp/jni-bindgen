// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-DatePickerDialog_OnDateSetListener"))]
__jni_bindgen! {
    /// public interface [DatePickerDialog.OnDateSetListener](https://developer.android.com/reference/android/app/DatePickerDialog.OnDateSetListener.html)
    ///
    /// Required feature: android-app-DatePickerDialog_OnDateSetListener
    public interface DatePickerDialog_OnDateSetListener ("android/app/DatePickerDialog$OnDateSetListener") extends crate::java::lang::Object {

        /// [onDateSet](https://developer.android.com/reference/android/app/DatePickerDialog.OnDateSetListener.html#onDateSet(android.widget.DatePicker,%20int,%20int,%20int))
        ///
        /// Required features: "android-widget-DatePicker"
        #[cfg(any(feature = "all", all(feature = "android-widget-DatePicker")))]
        pub fn onDateSet<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::DatePicker>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/DatePickerDialog$OnDateSetListener", java.flags == PUBLIC | ABSTRACT, .name == "onDateSet", .descriptor == "(Landroid/widget/DatePicker;III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/DatePickerDialog$OnDateSetListener\0", "onDateSet\0", "(Landroid/widget/DatePicker;III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
