// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-DialogInterface_OnDismissListener"))]
__jni_bindgen! {
    /// public interface [DialogInterface.OnDismissListener](https://developer.android.com/reference/android/content/DialogInterface.OnDismissListener.html)
    ///
    /// Required feature: android-content-DialogInterface_OnDismissListener
    public interface DialogInterface_OnDismissListener ("android/content/DialogInterface$OnDismissListener") extends crate::java::lang::Object {

        /// [onDismiss](https://developer.android.com/reference/android/content/DialogInterface.OnDismissListener.html#onDismiss(android.content.DialogInterface))
        ///
        /// Required features: "android-content-DialogInterface"
        #[cfg(any(feature = "all", all(feature = "android-content-DialogInterface")))]
        pub fn onDismiss<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/DialogInterface$OnDismissListener", java.flags == PUBLIC | ABSTRACT, .name == "onDismiss", .descriptor == "(Landroid/content/DialogInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/DialogInterface$OnDismissListener\0", "onDismiss\0", "(Landroid/content/DialogInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
