// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-Notification_Action_Extender"))]
__jni_bindgen! {
    /// public interface [Notification.Action.Extender](https://developer.android.com/reference/android/app/Notification.Action.Extender.html)
    ///
    /// Required feature: android-app-Notification_Action_Extender
    public interface Notification_Action_Extender ("android/app/Notification$Action$Extender") extends crate::java::lang::Object {

        /// [extend](https://developer.android.com/reference/android/app/Notification.Action.Extender.html#extend(android.app.Notification.Action.Builder))
        ///
        /// Required features: "android-app-Notification_Action_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_Action_Builder")))]
        pub fn extend<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Notification_Action_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_Action_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$Action$Extender", java.flags == PUBLIC | ABSTRACT, .name == "extend", .descriptor == "(Landroid/app/Notification$Action$Builder;)Landroid/app/Notification$Action$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$Action$Extender\0", "extend\0", "(Landroid/app/Notification$Action$Builder;)Landroid/app/Notification$Action$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
