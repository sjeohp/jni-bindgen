// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-gesture-GestureOverlayView_OnGesturePerformedListener"))]
__jni_bindgen! {
    /// public interface [GestureOverlayView.OnGesturePerformedListener](https://developer.android.com/reference/android/gesture/GestureOverlayView.OnGesturePerformedListener.html)
    ///
    /// Required feature: android-gesture-GestureOverlayView_OnGesturePerformedListener
    public interface GestureOverlayView_OnGesturePerformedListener ("android/gesture/GestureOverlayView$OnGesturePerformedListener") extends crate::java::lang::Object {

        /// [onGesturePerformed](https://developer.android.com/reference/android/gesture/GestureOverlayView.OnGesturePerformedListener.html#onGesturePerformed(android.gesture.GestureOverlayView,%20android.gesture.Gesture))
        ///
        /// Required features: "android-gesture-Gesture", "android-gesture-GestureOverlayView"
        #[cfg(any(feature = "all", all(feature = "android-gesture-Gesture", feature = "android-gesture-GestureOverlayView")))]
        pub fn onGesturePerformed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::GestureOverlayView>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::gesture::Gesture>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/gesture/GestureOverlayView$OnGesturePerformedListener", java.flags == PUBLIC | ABSTRACT, .name == "onGesturePerformed", .descriptor == "(Landroid/gesture/GestureOverlayView;Landroid/gesture/Gesture;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/gesture/GestureOverlayView$OnGesturePerformedListener\0", "onGesturePerformed\0", "(Landroid/gesture/GestureOverlayView;Landroid/gesture/Gesture;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
