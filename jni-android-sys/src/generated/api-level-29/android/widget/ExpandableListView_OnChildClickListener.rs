// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-ExpandableListView_OnChildClickListener"))]
__jni_bindgen! {
    /// public interface [ExpandableListView.OnChildClickListener](https://developer.android.com/reference/android/widget/ExpandableListView.OnChildClickListener.html)
    ///
    /// Required feature: android-widget-ExpandableListView_OnChildClickListener
    public interface ExpandableListView_OnChildClickListener ("android/widget/ExpandableListView$OnChildClickListener") extends crate::java::lang::Object {

        /// [onChildClick](https://developer.android.com/reference/android/widget/ExpandableListView.OnChildClickListener.html#onChildClick(android.widget.ExpandableListView,%20android.view.View,%20int,%20int,%20long))
        ///
        /// Required features: "android-view-View", "android-widget-ExpandableListView"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-widget-ExpandableListView")))]
        pub fn onChildClick<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::widget::ExpandableListView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg2: i32, arg3: i32, arg4: i64) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ExpandableListView$OnChildClickListener", java.flags == PUBLIC | ABSTRACT, .name == "onChildClick", .descriptor == "(Landroid/widget/ExpandableListView;Landroid/view/View;IIJ)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ExpandableListView$OnChildClickListener\0", "onChildClick\0", "(Landroid/widget/ExpandableListView;Landroid/view/View;IIJ)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
