// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-drm-DrmManagerClient_OnErrorListener"))]
__jni_bindgen! {
    /// public interface [DrmManagerClient.OnErrorListener](https://developer.android.com/reference/android/drm/DrmManagerClient.OnErrorListener.html)
    ///
    /// Required feature: android-drm-DrmManagerClient_OnErrorListener
    public interface DrmManagerClient_OnErrorListener ("android/drm/DrmManagerClient$OnErrorListener") extends crate::java::lang::Object {

        /// [onError](https://developer.android.com/reference/android/drm/DrmManagerClient.OnErrorListener.html#onError(android.drm.DrmManagerClient,%20android.drm.DrmErrorEvent))
        ///
        /// Required features: "android-drm-DrmErrorEvent", "android-drm-DrmManagerClient"
        #[cfg(any(feature = "all", all(feature = "android-drm-DrmErrorEvent", feature = "android-drm-DrmManagerClient")))]
        pub fn onError<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::drm::DrmManagerClient>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::drm::DrmErrorEvent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/drm/DrmManagerClient$OnErrorListener", java.flags == PUBLIC | ABSTRACT, .name == "onError", .descriptor == "(Landroid/drm/DrmManagerClient;Landroid/drm/DrmErrorEvent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/drm/DrmManagerClient$OnErrorListener\0", "onError\0", "(Landroid/drm/DrmManagerClient;Landroid/drm/DrmErrorEvent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
