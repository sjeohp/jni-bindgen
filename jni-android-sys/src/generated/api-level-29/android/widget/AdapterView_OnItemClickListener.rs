// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-AdapterView_OnItemClickListener"))]
__jni_bindgen! {
    /// public interface [AdapterView.OnItemClickListener](https://developer.android.com/reference/android/widget/AdapterView.OnItemClickListener.html)
    ///
    /// Required feature: android-widget-AdapterView_OnItemClickListener
    public interface AdapterView_OnItemClickListener ("android/widget/AdapterView$OnItemClickListener") extends crate::java::lang::Object {

        /// [onItemClick](https://developer.android.com/reference/android/widget/AdapterView.OnItemClickListener.html#onItemClick(android.widget.AdapterView,%20android.view.View,%20int,%20long))
        ///
        /// Required features: "android-view-View", "android-widget-AdapterView"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-widget-AdapterView")))]
        pub fn onItemClick<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::AdapterView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: i32, arg3: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/AdapterView$OnItemClickListener", java.flags == PUBLIC | ABSTRACT, .name == "onItemClick", .descriptor == "(Landroid/widget/AdapterView;Landroid/view/View;IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/AdapterView$OnItemClickListener\0", "onItemClick\0", "(Landroid/widget/AdapterView;Landroid/view/View;IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
