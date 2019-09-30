// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-accessibility-CaptioningManager_CaptioningChangeListener"))]
__jni_bindgen! {
    /// public class [CaptioningManager.CaptioningChangeListener](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html)
    ///
    /// Required feature: android-view-accessibility-CaptioningManager_CaptioningChangeListener
    public class CaptioningManager_CaptioningChangeListener ("android/view/accessibility/CaptioningManager$CaptioningChangeListener") extends crate::java::lang::Object {

        /// [CaptioningChangeListener](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html#CaptioningChangeListener())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::view::accessibility::CaptioningManager_CaptioningChangeListener>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/CaptioningManager$CaptioningChangeListener", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/CaptioningManager$CaptioningChangeListener\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onEnabledChanged](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html#onEnabledChanged(boolean))
        pub fn onEnabledChanged<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/CaptioningManager$CaptioningChangeListener", java.flags == PUBLIC, .name == "onEnabledChanged", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/CaptioningManager$CaptioningChangeListener\0", "onEnabledChanged\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onUserStyleChanged](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html#onUserStyleChanged(android.view.accessibility.CaptioningManager.CaptionStyle))
        ///
        /// Required features: "android-view-accessibility-CaptioningManager_CaptionStyle"
        #[cfg(any(feature = "all", all(feature = "android-view-accessibility-CaptioningManager_CaptionStyle")))]
        pub fn onUserStyleChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::accessibility::CaptioningManager_CaptionStyle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/CaptioningManager$CaptioningChangeListener", java.flags == PUBLIC, .name == "onUserStyleChanged", .descriptor == "(Landroid/view/accessibility/CaptioningManager$CaptionStyle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/CaptioningManager$CaptioningChangeListener\0", "onUserStyleChanged\0", "(Landroid/view/accessibility/CaptioningManager$CaptionStyle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLocaleChanged](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html#onLocaleChanged(java.util.Locale))
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn onLocaleChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/CaptioningManager$CaptioningChangeListener", java.flags == PUBLIC, .name == "onLocaleChanged", .descriptor == "(Ljava/util/Locale;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/CaptioningManager$CaptioningChangeListener\0", "onLocaleChanged\0", "(Ljava/util/Locale;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFontScaleChanged](https://developer.android.com/reference/android/view/accessibility/CaptioningManager.CaptioningChangeListener.html#onFontScaleChanged(float))
        pub fn onFontScaleChanged<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/accessibility/CaptioningManager$CaptioningChangeListener", java.flags == PUBLIC, .name == "onFontScaleChanged", .descriptor == "(F)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/accessibility/CaptioningManager$CaptioningChangeListener\0", "onFontScaleChanged\0", "(F)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
