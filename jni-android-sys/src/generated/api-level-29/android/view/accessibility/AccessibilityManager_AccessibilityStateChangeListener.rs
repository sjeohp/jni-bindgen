// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-accessibility-AccessibilityManager_AccessibilityStateChangeListener"))]
__jni_bindgen! {
    /// public interface [AccessibilityManager.AccessibilityStateChangeListener](https://developer.android.com/reference/android/view/accessibility/AccessibilityManager.AccessibilityStateChangeListener.html)
    ///
    /// Required feature: android-view-accessibility-AccessibilityManager_AccessibilityStateChangeListener
    public interface AccessibilityManager_AccessibilityStateChangeListener ("android/view/accessibility/AccessibilityManager$AccessibilityStateChangeListener") extends crate::java::lang::Object {

        /// [onAccessibilityStateChanged](https://developer.android.com/reference/android/view/accessibility/AccessibilityManager.AccessibilityStateChangeListener.html#onAccessibilityStateChanged(boolean))
        pub fn onAccessibilityStateChanged<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/AccessibilityManager$AccessibilityStateChangeListener", java.flags == PUBLIC | ABSTRACT, .name == "onAccessibilityStateChanged", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/AccessibilityManager$AccessibilityStateChangeListener\0", "onAccessibilityStateChanged\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
