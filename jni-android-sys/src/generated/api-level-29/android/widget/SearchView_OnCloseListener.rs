// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-SearchView_OnCloseListener"))]
__jni_bindgen! {
    /// public interface [SearchView.OnCloseListener](https://developer.android.com/reference/android/widget/SearchView.OnCloseListener.html)
    ///
    /// Required feature: android-widget-SearchView_OnCloseListener
    public interface SearchView_OnCloseListener ("android/widget/SearchView$OnCloseListener") extends crate::java::lang::Object {

        /// [onClose](https://developer.android.com/reference/android/widget/SearchView.OnCloseListener.html#onClose())
        pub fn onClose<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/SearchView$OnCloseListener", java.flags == PUBLIC | ABSTRACT, .name == "onClose", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/SearchView$OnCloseListener\0", "onClose\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
