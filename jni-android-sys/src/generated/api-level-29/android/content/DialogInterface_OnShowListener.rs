// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-DialogInterface_OnShowListener"))]
__jni_bindgen! {
    /// public interface [DialogInterface.OnShowListener](https://developer.android.com/reference/android/content/DialogInterface.OnShowListener.html)
    ///
    /// Required feature: android-content-DialogInterface_OnShowListener
    public interface DialogInterface_OnShowListener ("android/content/DialogInterface$OnShowListener") extends crate::java::lang::Object {

        /// [onShow](https://developer.android.com/reference/android/content/DialogInterface.OnShowListener.html#onShow(android.content.DialogInterface))
        ///
        /// Required features: "android-content-DialogInterface"
        #[cfg(any(feature = "all", all(feature = "android-content-DialogInterface")))]
        pub fn onShow<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::DialogInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/DialogInterface$OnShowListener", java.flags == PUBLIC | ABSTRACT, .name == "onShow", .descriptor == "(Landroid/content/DialogInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/DialogInterface$OnShowListener\0", "onShow\0", "(Landroid/content/DialogInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
