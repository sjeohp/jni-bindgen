// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener"))]
__jni_bindgen! {
    /// public interface [AccessibilityService.SoftKeyboardController.OnShowModeChangedListener](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.OnShowModeChangedListener.html)
    ///
    /// Required feature: android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener
    public interface AccessibilityService_SoftKeyboardController_OnShowModeChangedListener ("android/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener") extends crate::java::lang::Object {

        /// [onShowModeChanged](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.OnShowModeChangedListener.html#onShowModeChanged(android.accessibilityservice.AccessibilityService.SoftKeyboardController,%20int))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityService_SoftKeyboardController"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController")))]
        pub fn onShowModeChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityService_SoftKeyboardController>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener", java.flags == PUBLIC | ABSTRACT, .name == "onShowModeChanged", .descriptor == "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener\0", "onShowModeChanged\0", "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
