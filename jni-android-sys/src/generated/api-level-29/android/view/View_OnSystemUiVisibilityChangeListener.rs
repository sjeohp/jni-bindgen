// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-View_OnSystemUiVisibilityChangeListener"))]
__jni_bindgen! {
    /// public interface [View.OnSystemUiVisibilityChangeListener](https://developer.android.com/reference/android/view/View.OnSystemUiVisibilityChangeListener.html)
    ///
    /// Required feature: android-view-View_OnSystemUiVisibilityChangeListener
    public interface View_OnSystemUiVisibilityChangeListener ("android/view/View$OnSystemUiVisibilityChangeListener") extends crate::java::lang::Object {

        /// [onSystemUiVisibilityChange](https://developer.android.com/reference/android/view/View.OnSystemUiVisibilityChangeListener.html#onSystemUiVisibilityChange(int))
        pub fn onSystemUiVisibilityChange<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/View$OnSystemUiVisibilityChangeListener", java.flags == PUBLIC | ABSTRACT, .name == "onSystemUiVisibilityChange", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/View$OnSystemUiVisibilityChangeListener\0", "onSystemUiVisibilityChange\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
