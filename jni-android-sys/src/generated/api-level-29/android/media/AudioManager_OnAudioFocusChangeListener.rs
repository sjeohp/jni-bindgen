// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-AudioManager_OnAudioFocusChangeListener"))]
__jni_bindgen! {
    /// public interface [AudioManager.OnAudioFocusChangeListener](https://developer.android.com/reference/android/media/AudioManager.OnAudioFocusChangeListener.html)
    ///
    /// Required feature: android-media-AudioManager_OnAudioFocusChangeListener
    public interface AudioManager_OnAudioFocusChangeListener ("android/media/AudioManager$OnAudioFocusChangeListener") extends crate::java::lang::Object {

        /// [onAudioFocusChange](https://developer.android.com/reference/android/media/AudioManager.OnAudioFocusChangeListener.html#onAudioFocusChange(int))
        pub fn onAudioFocusChange<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioManager$OnAudioFocusChangeListener", java.flags == PUBLIC | ABSTRACT, .name == "onAudioFocusChange", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioManager$OnAudioFocusChangeListener\0", "onAudioFocusChange\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
