// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-AbsListView_MultiChoiceModeListener"))]
__jni_bindgen! {
    /// public interface [AbsListView.MultiChoiceModeListener](https://developer.android.com/reference/android/widget/AbsListView.MultiChoiceModeListener.html)
    ///
    /// Required feature: android-widget-AbsListView_MultiChoiceModeListener
    public interface AbsListView_MultiChoiceModeListener ("android/widget/AbsListView$MultiChoiceModeListener") extends crate::java::lang::Object, implements crate::android::view::ActionMode_Callback {

        /// [onItemCheckedStateChanged](https://developer.android.com/reference/android/widget/AbsListView.MultiChoiceModeListener.html#onItemCheckedStateChanged(android.view.ActionMode,%20int,%20long,%20boolean))
        ///
        /// Required features: "android-view-ActionMode"
        #[cfg(any(feature = "all", all(feature = "android-view-ActionMode")))]
        pub fn onItemCheckedStateChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::ActionMode>>, arg1: i32, arg2: i64, arg3: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/AbsListView$MultiChoiceModeListener", java.flags == PUBLIC | ABSTRACT, .name == "onItemCheckedStateChanged", .descriptor == "(Landroid/view/ActionMode;IJZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/AbsListView$MultiChoiceModeListener\0", "onItemCheckedStateChanged\0", "(Landroid/view/ActionMode;IJZ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
