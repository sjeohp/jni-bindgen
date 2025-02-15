// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-View_OnApplyWindowInsetsListener"))]
__jni_bindgen! {
    /// public interface [View.OnApplyWindowInsetsListener](https://developer.android.com/reference/android/view/View.OnApplyWindowInsetsListener.html)
    ///
    /// Required feature: android-view-View_OnApplyWindowInsetsListener
    public interface View_OnApplyWindowInsetsListener ("android/view/View$OnApplyWindowInsetsListener") extends crate::java::lang::Object {

        /// [onApplyWindowInsets](https://developer.android.com/reference/android/view/View.OnApplyWindowInsetsListener.html#onApplyWindowInsets(android.view.View,%20android.view.WindowInsets))
        ///
        /// Required features: "android-view-View", "android-view-WindowInsets"
        #[cfg(any(feature = "all", all(feature = "android-view-View", feature = "android-view-WindowInsets")))]
        pub fn onApplyWindowInsets<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::View>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::WindowInsets>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::WindowInsets>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$OnApplyWindowInsetsListener", java.flags == PUBLIC | ABSTRACT, .name == "onApplyWindowInsets", .descriptor == "(Landroid/view/View;Landroid/view/WindowInsets;)Landroid/view/WindowInsets;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/View$OnApplyWindowInsetsListener\0", "onApplyWindowInsets\0", "(Landroid/view/View;Landroid/view/WindowInsets;)Landroid/view/WindowInsets;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
