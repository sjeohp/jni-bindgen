// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-StatusBarManager"))]
__jni_bindgen! {
    /// public class [StatusBarManager](https://developer.android.com/reference/android/app/StatusBarManager.html)
    ///
    /// Required feature: android-app-StatusBarManager
    public class StatusBarManager ("android/app/StatusBarManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [StatusBarManager](https://developer.android.com/reference/android/app/StatusBarManager.html#StatusBarManager(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::StatusBarManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/app/StatusBarManager", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/StatusBarManager\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
