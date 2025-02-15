// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-view-InputQueue_Callback"))]
__jni_bindgen! {
    /// public interface [InputQueue.Callback](https://developer.android.com/reference/android/view/InputQueue.Callback.html)
    ///
    /// Required feature: android-view-InputQueue_Callback
    public interface InputQueue_Callback ("android/view/InputQueue$Callback") extends crate::java::lang::Object {

        /// [onInputQueueCreated](https://developer.android.com/reference/android/view/InputQueue.Callback.html#onInputQueueCreated(android.view.InputQueue))
        ///
        /// Required features: "android-view-InputQueue"
        #[cfg(any(feature = "all", all(feature = "android-view-InputQueue")))]
        pub fn onInputQueueCreated<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::InputQueue>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputQueue$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onInputQueueCreated", .descriptor == "(Landroid/view/InputQueue;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputQueue$Callback\0", "onInputQueueCreated\0", "(Landroid/view/InputQueue;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onInputQueueDestroyed](https://developer.android.com/reference/android/view/InputQueue.Callback.html#onInputQueueDestroyed(android.view.InputQueue))
        ///
        /// Required features: "android-view-InputQueue"
        #[cfg(any(feature = "all", all(feature = "android-view-InputQueue")))]
        pub fn onInputQueueDestroyed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::InputQueue>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/view/InputQueue$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onInputQueueDestroyed", .descriptor == "(Landroid/view/InputQueue;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/view/InputQueue$Callback\0", "onInputQueueDestroyed\0", "(Landroid/view/InputQueue;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
