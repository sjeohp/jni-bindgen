// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaPlayer_OnSubtitleDataListener"))]
__jni_bindgen! {
    /// public interface [MediaPlayer.OnSubtitleDataListener](https://developer.android.com/reference/android/media/MediaPlayer.OnSubtitleDataListener.html)
    ///
    /// Required feature: android-media-MediaPlayer_OnSubtitleDataListener
    public interface MediaPlayer_OnSubtitleDataListener ("android/media/MediaPlayer$OnSubtitleDataListener") extends crate::java::lang::Object {

        /// [onSubtitleData](https://developer.android.com/reference/android/media/MediaPlayer.OnSubtitleDataListener.html#onSubtitleData(android.media.MediaPlayer,%20android.media.SubtitleData))
        ///
        /// Required features: "android-media-MediaPlayer", "android-media-SubtitleData"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaPlayer", feature = "android-media-SubtitleData")))]
        pub fn onSubtitleData<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaPlayer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::SubtitleData>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaPlayer$OnSubtitleDataListener", java.flags == PUBLIC | ABSTRACT, .name == "onSubtitleData", .descriptor == "(Landroid/media/MediaPlayer;Landroid/media/SubtitleData;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaPlayer$OnSubtitleDataListener\0", "onSubtitleData\0", "(Landroid/media/MediaPlayer;Landroid/media/SubtitleData;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
