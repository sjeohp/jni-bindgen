// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-content-Loader_OnLoadCanceledListener"))]
__jni_bindgen! {
    /// public interface [Loader.OnLoadCanceledListener](https://developer.android.com/reference/android/content/Loader.OnLoadCanceledListener.html)
    ///
    /// Required feature: android-content-Loader_OnLoadCanceledListener
    #[deprecated] public interface Loader_OnLoadCanceledListener ("android/content/Loader$OnLoadCanceledListener") extends crate::java::lang::Object {

        /// [onLoadCanceled](https://developer.android.com/reference/android/content/Loader.OnLoadCanceledListener.html#onLoadCanceled(android.content.Loader))
        ///
        /// Required features: "android-content-Loader"
        #[cfg(any(feature = "all", all(feature = "android-content-Loader")))]
        #[deprecated] pub fn onLoadCanceled<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Loader>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/content/Loader$OnLoadCanceledListener", java.flags == PUBLIC | ABSTRACT, .name == "onLoadCanceled", .descriptor == "(Landroid/content/Loader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/content/Loader$OnLoadCanceledListener\0", "onLoadCanceled\0", "(Landroid/content/Loader;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
