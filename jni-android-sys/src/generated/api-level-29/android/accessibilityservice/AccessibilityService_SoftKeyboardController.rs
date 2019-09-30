// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController"))]
__jni_bindgen! {
    /// public final class [AccessibilityService.SoftKeyboardController](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html)
    ///
    /// Required feature: android-accessibilityservice-AccessibilityService_SoftKeyboardController
    public final class AccessibilityService_SoftKeyboardController ("android/accessibilityservice/AccessibilityService$SoftKeyboardController") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SoftKeyboardController](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#SoftKeyboardController(android.accessibilityservice.AccessibilityService,%20java.lang.Object))
        // ///
        // /// Required features: "android-accessibilityservice-AccessibilityService", "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityService", feature = "java-lang-Object")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityService>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::accessibilityservice::AccessibilityService_SoftKeyboardController>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/accessibilityservice/AccessibilityService;Ljava/lang/Object;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "<init>\0", "(Landroid/accessibilityservice/AccessibilityService;Ljava/lang/Object;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [addOnShowModeChangedListener](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#addOnShowModeChangedListener(android.accessibilityservice.AccessibilityService.SoftKeyboardController.OnShowModeChangedListener))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener")))]
        pub fn addOnShowModeChangedListener_OnShowModeChangedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityService_SoftKeyboardController_OnShowModeChangedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == PUBLIC, .name == "addOnShowModeChangedListener", .descriptor == "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "addOnShowModeChangedListener\0", "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnShowModeChangedListener](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#addOnShowModeChangedListener(android.accessibilityservice.AccessibilityService.SoftKeyboardController.OnShowModeChangedListener,%20android.os.Handler))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener", feature = "android-os-Handler")))]
        pub fn addOnShowModeChangedListener_OnShowModeChangedListener_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityService_SoftKeyboardController_OnShowModeChangedListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == PUBLIC, .name == "addOnShowModeChangedListener", .descriptor == "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "addOnShowModeChangedListener\0", "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeOnShowModeChangedListener](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#removeOnShowModeChangedListener(android.accessibilityservice.AccessibilityService.SoftKeyboardController.OnShowModeChangedListener))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityService_SoftKeyboardController_OnShowModeChangedListener")))]
        pub fn removeOnShowModeChangedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityService_SoftKeyboardController_OnShowModeChangedListener>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == PUBLIC, .name == "removeOnShowModeChangedListener", .descriptor == "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "removeOnShowModeChangedListener\0", "(Landroid/accessibilityservice/AccessibilityService$SoftKeyboardController$OnShowModeChangedListener;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getShowMode](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#getShowMode())
        pub fn getShowMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == PUBLIC, .name == "getShowMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "getShowMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShowMode](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService.SoftKeyboardController.html#setShowMode(int))
        pub fn setShowMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityService$SoftKeyboardController", java.flags == PUBLIC, .name == "setShowMode", .descriptor == "(I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityService$SoftKeyboardController\0", "setShowMode\0", "(I)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
