// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-AdapterView_OnItemSelectedListener"))]
__jni_bindgen! {
    /// public interface [AdapterView.OnItemSelectedListener](https://developer.android.com/reference/android/widget/AdapterView.OnItemSelectedListener.html)
    ///
    /// Required feature: android-widget-AdapterView_OnItemSelectedListener
    public interface AdapterView_OnItemSelectedListener ("android/widget/AdapterView$OnItemSelectedListener") extends crate::java::lang::Object {

        /// [onItemSelected](https://developer.android.com/reference/android/widget/AdapterView.OnItemSelectedListener.html#onItemSelected(android.widget.AdapterView,%20android.view.View,%20int,%20long))
        ///
        /// Required features: "android-view-View", "android-widget-AdapterView"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-widget-AdapterView")))]
        pub fn onItemSelected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::AdapterView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: i32, arg3: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/AdapterView$OnItemSelectedListener", java.flags == PUBLIC | ABSTRACT, .name == "onItemSelected", .descriptor == "(Landroid/widget/AdapterView;Landroid/view/View;IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/AdapterView$OnItemSelectedListener\0", "onItemSelected\0", "(Landroid/widget/AdapterView;Landroid/view/View;IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onNothingSelected](https://developer.android.com/reference/android/widget/AdapterView.OnItemSelectedListener.html#onNothingSelected(android.widget.AdapterView))
        ///
        /// Required features: "android-widget-AdapterView"
        #[cfg(any(feature = "all", all(feature = "android-widget-AdapterView")))]
        pub fn onNothingSelected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::AdapterView>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/AdapterView$OnItemSelectedListener", java.flags == PUBLIC | ABSTRACT, .name == "onNothingSelected", .descriptor == "(Landroid/widget/AdapterView;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/AdapterView$OnItemSelectedListener\0", "onNothingSelected\0", "(Landroid/widget/AdapterView;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
