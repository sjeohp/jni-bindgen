// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-KeyEvent_Callback"))]
__jni_bindgen! {
    /// public interface [KeyEvent.Callback](https://developer.android.com/reference/android/view/KeyEvent.Callback.html)
    ///
    /// Required feature: android-view-KeyEvent_Callback
    public interface KeyEvent_Callback ("android/view/KeyEvent$Callback") extends crate::java::lang::Object {

        /// [onKeyDown](https://developer.android.com/reference/android/view/KeyEvent.Callback.html#onKeyDown(int,%20android.view.KeyEvent))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn onKeyDown<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyEvent$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onKeyDown", .descriptor == "(ILandroid/view/KeyEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyEvent$Callback\0", "onKeyDown\0", "(ILandroid/view/KeyEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onKeyLongPress](https://developer.android.com/reference/android/view/KeyEvent.Callback.html#onKeyLongPress(int,%20android.view.KeyEvent))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn onKeyLongPress<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyEvent$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onKeyLongPress", .descriptor == "(ILandroid/view/KeyEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyEvent$Callback\0", "onKeyLongPress\0", "(ILandroid/view/KeyEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onKeyUp](https://developer.android.com/reference/android/view/KeyEvent.Callback.html#onKeyUp(int,%20android.view.KeyEvent))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn onKeyUp<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyEvent$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onKeyUp", .descriptor == "(ILandroid/view/KeyEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyEvent$Callback\0", "onKeyUp\0", "(ILandroid/view/KeyEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onKeyMultiple](https://developer.android.com/reference/android/view/KeyEvent.Callback.html#onKeyMultiple(int,%20int,%20android.view.KeyEvent))
        ///
        /// Required features: "android-view-KeyEvent"
        #[cfg(any(feature = "all", all(feature = "android-view-KeyEvent")))]
        pub fn onKeyMultiple<'env>(&'env self, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::KeyEvent>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/KeyEvent$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onKeyMultiple", .descriptor == "(IILandroid/view/KeyEvent;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/KeyEvent$Callback\0", "onKeyMultiple\0", "(IILandroid/view/KeyEvent;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
