// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-accessibilityservice-AccessibilityButtonController"))]
__jni_bindgen! {
    /// public final class [AccessibilityButtonController](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html)
    ///
    /// Required feature: android-accessibilityservice-AccessibilityButtonController
    public final class AccessibilityButtonController ("android/accessibilityservice/AccessibilityButtonController") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [AccessibilityButtonController](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html#AccessibilityButtonController())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::accessibilityservice::AccessibilityButtonController>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/accessibilityservice/AccessibilityButtonController", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityButtonController\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isAccessibilityButtonAvailable](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html#isAccessibilityButtonAvailable())
        pub fn isAccessibilityButtonAvailable<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityButtonController", java.flags == PUBLIC, .name == "isAccessibilityButtonAvailable", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityButtonController\0", "isAccessibilityButtonAvailable\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerAccessibilityButtonCallback](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html#registerAccessibilityButtonCallback(android.accessibilityservice.AccessibilityButtonController.AccessibilityButtonCallback))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback")))]
        pub fn registerAccessibilityButtonCallback_AccessibilityButtonCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityButtonController_AccessibilityButtonCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityButtonController", java.flags == PUBLIC, .name == "registerAccessibilityButtonCallback", .descriptor == "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityButtonController\0", "registerAccessibilityButtonCallback\0", "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [registerAccessibilityButtonCallback](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html#registerAccessibilityButtonCallback(android.accessibilityservice.AccessibilityButtonController.AccessibilityButtonCallback,%20android.os.Handler))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback", feature = "android-os-Handler")))]
        pub fn registerAccessibilityButtonCallback_AccessibilityButtonCallback_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityButtonController_AccessibilityButtonCallback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityButtonController", java.flags == PUBLIC, .name == "registerAccessibilityButtonCallback", .descriptor == "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityButtonController\0", "registerAccessibilityButtonCallback\0", "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unregisterAccessibilityButtonCallback](https://developer.android.com/reference/android/accessibilityservice/AccessibilityButtonController.html#unregisterAccessibilityButtonCallback(android.accessibilityservice.AccessibilityButtonController.AccessibilityButtonCallback))
        ///
        /// Required features: "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback"
        #[cfg(any(feature = "all", all(feature = "android-accessibilityservice-AccessibilityButtonController_AccessibilityButtonCallback")))]
        pub fn unregisterAccessibilityButtonCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::accessibilityservice::AccessibilityButtonController_AccessibilityButtonCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/accessibilityservice/AccessibilityButtonController", java.flags == PUBLIC, .name == "unregisterAccessibilityButtonCallback", .descriptor == "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/accessibilityservice/AccessibilityButtonController\0", "unregisterAccessibilityButtonCallback\0", "(Landroid/accessibilityservice/AccessibilityButtonController$AccessibilityButtonCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
