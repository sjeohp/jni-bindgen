// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-ViewSwitcher_ViewFactory"))]
__jni_bindgen! {
    /// public interface [ViewSwitcher.ViewFactory](https://developer.android.com/reference/android/widget/ViewSwitcher.ViewFactory.html)
    ///
    /// Required feature: android-widget-ViewSwitcher_ViewFactory
    public interface ViewSwitcher_ViewFactory ("android/widget/ViewSwitcher$ViewFactory") extends crate::java::lang::Object {

        /// [makeView](https://developer.android.com/reference/android/widget/ViewSwitcher.ViewFactory.html#makeView())
        ///
        /// Required features: "android-view-View"
        #[cfg(any(feature = "all", all(feature = "android-view-View")))]
        pub fn makeView<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::View>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ViewSwitcher$ViewFactory", java.flags == PUBLIC | ABSTRACT, .name == "makeView", .descriptor == "()Landroid/view/View;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ViewSwitcher$ViewFactory\0", "makeView\0", "()Landroid/view/View;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
