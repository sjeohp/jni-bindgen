// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaSync_OnErrorListener"))]
__jni_bindgen! {
    /// public interface [MediaSync.OnErrorListener](https://developer.android.com/reference/android/media/MediaSync.OnErrorListener.html)
    ///
    /// Required feature: android-media-MediaSync_OnErrorListener
    public interface MediaSync_OnErrorListener ("android/media/MediaSync$OnErrorListener") extends crate::java::lang::Object {

        /// [onError](https://developer.android.com/reference/android/media/MediaSync.OnErrorListener.html#onError(android.media.MediaSync,%20int,%20int))
        ///
        /// Required features: "android-media-MediaSync"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaSync")))]
        pub fn onError<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaSync>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync$OnErrorListener", java.flags == PUBLIC | ABSTRACT, .name == "onError", .descriptor == "(Landroid/media/MediaSync;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync$OnErrorListener\0", "onError\0", "(Landroid/media/MediaSync;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
