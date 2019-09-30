// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-AudioManager_AudioRecordingCallback"))]
__jni_bindgen! {
    /// public class [AudioManager.AudioRecordingCallback](https://developer.android.com/reference/android/media/AudioManager.AudioRecordingCallback.html)
    ///
    /// Required feature: android-media-AudioManager_AudioRecordingCallback
    public class AudioManager_AudioRecordingCallback ("android/media/AudioManager$AudioRecordingCallback") extends crate::java::lang::Object {

        /// [AudioRecordingCallback](https://developer.android.com/reference/android/media/AudioManager.AudioRecordingCallback.html#AudioRecordingCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::AudioManager_AudioRecordingCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioManager$AudioRecordingCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioManager$AudioRecordingCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRecordingConfigChanged](https://developer.android.com/reference/android/media/AudioManager.AudioRecordingCallback.html#onRecordingConfigChanged(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn onRecordingConfigChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioManager$AudioRecordingCallback", java.flags == PUBLIC, .name == "onRecordingConfigChanged", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioManager$AudioRecordingCallback\0", "onRecordingConfigChanged\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
