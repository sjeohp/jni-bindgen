// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-widget-ZoomButtonsController_OnZoomListener"))]
__jni_bindgen! {
    /// public interface [ZoomButtonsController.OnZoomListener](https://developer.android.com/reference/android/widget/ZoomButtonsController.OnZoomListener.html)
    ///
    /// Required feature: android-widget-ZoomButtonsController_OnZoomListener
    #[deprecated] public interface ZoomButtonsController_OnZoomListener ("android/widget/ZoomButtonsController$OnZoomListener") extends crate::java::lang::Object {

        /// [onVisibilityChanged](https://developer.android.com/reference/android/widget/ZoomButtonsController.OnZoomListener.html#onVisibilityChanged(boolean))
        #[deprecated] pub fn onVisibilityChanged<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ZoomButtonsController$OnZoomListener", java.flags == PUBLIC | ABSTRACT, .name == "onVisibilityChanged", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ZoomButtonsController$OnZoomListener\0", "onVisibilityChanged\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onZoom](https://developer.android.com/reference/android/widget/ZoomButtonsController.OnZoomListener.html#onZoom(boolean))
        #[deprecated] pub fn onZoom<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/widget/ZoomButtonsController$OnZoomListener", java.flags == PUBLIC | ABSTRACT, .name == "onZoom", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/widget/ZoomButtonsController$OnZoomListener\0", "onZoom\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
