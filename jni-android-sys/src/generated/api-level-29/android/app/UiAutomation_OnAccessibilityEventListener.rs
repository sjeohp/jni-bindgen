// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-UiAutomation_OnAccessibilityEventListener"))]
__jni_bindgen! {
    /// public interface [UiAutomation.OnAccessibilityEventListener](https://developer.android.com/reference/android/app/UiAutomation.OnAccessibilityEventListener.html)
    ///
    /// Required feature: android-app-UiAutomation_OnAccessibilityEventListener
    public interface UiAutomation_OnAccessibilityEventListener ("android/app/UiAutomation$OnAccessibilityEventListener") extends crate::java::lang::Object {

        /// [onAccessibilityEvent](https://developer.android.com/reference/android/app/UiAutomation.OnAccessibilityEventListener.html#onAccessibilityEvent(android.view.accessibility.AccessibilityEvent))
        ///
        /// Required features: "android-view-accessibility-AccessibilityEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-accessibility-AccessibilityEvent")))]
        pub fn onAccessibilityEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::accessibility::AccessibilityEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/UiAutomation$OnAccessibilityEventListener", java.flags == PUBLIC | ABSTRACT, .name == "onAccessibilityEvent", .descriptor == "(Landroid/view/accessibility/AccessibilityEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/UiAutomation$OnAccessibilityEventListener\0", "onAccessibilityEvent\0", "(Landroid/view/accessibility/AccessibilityEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
