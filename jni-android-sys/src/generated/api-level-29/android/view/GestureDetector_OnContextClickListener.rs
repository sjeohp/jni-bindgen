// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-GestureDetector_OnContextClickListener"))]
__jni_bindgen! {
    /// public interface [GestureDetector.OnContextClickListener](https://developer.android.com/reference/android/view/GestureDetector.OnContextClickListener.html)
    ///
    /// Required feature: android-view-GestureDetector_OnContextClickListener
    public interface GestureDetector_OnContextClickListener ("android/view/GestureDetector$OnContextClickListener") extends crate::java::lang::Object {

        /// [onContextClick](https://developer.android.com/reference/android/view/GestureDetector.OnContextClickListener.html#onContextClick(android.view.MotionEvent))
        ///
        /// Required features: "android-view-MotionEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-MotionEvent")))]
        pub fn onContextClick<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::MotionEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/GestureDetector$OnContextClickListener", java.flags == PUBLIC | ABSTRACT, .name == "onContextClick", .descriptor == "(Landroid/view/MotionEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/GestureDetector$OnContextClickListener\0", "onContextClick\0", "(Landroid/view/MotionEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
