// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaCodec_OnFrameRenderedListener"))]
__jni_bindgen! {
    /// public interface [MediaCodec.OnFrameRenderedListener](https://developer.android.com/reference/android/media/MediaCodec.OnFrameRenderedListener.html)
    ///
    /// Required feature: android-media-MediaCodec_OnFrameRenderedListener
    public interface MediaCodec_OnFrameRenderedListener ("android/media/MediaCodec$OnFrameRenderedListener") extends crate::java::lang::Object {

        /// [onFrameRendered](https://developer.android.com/reference/android/media/MediaCodec.OnFrameRenderedListener.html#onFrameRendered(android.media.MediaCodec,%20long,%20long))
        ///
        /// Required features: "android-media-MediaCodec"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaCodec")))]
        pub fn onFrameRendered<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaCodec>>, arg1: i64, arg2: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaCodec$OnFrameRenderedListener", java.flags == PUBLIC | ABSTRACT, .name == "onFrameRendered", .descriptor == "(Landroid/media/MediaCodec;JJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaCodec$OnFrameRenderedListener\0", "onFrameRendered\0", "(Landroid/media/MediaCodec;JJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
