// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-session-MediaController_Callback"))]
__jni_bindgen! {
    /// public class [MediaController.Callback](https://developer.android.com/reference/android/media/session/MediaController.Callback.html)
    ///
    /// Required feature: android-media-session-MediaController_Callback
    public class MediaController_Callback ("android/media/session/MediaController$Callback") extends crate::java::lang::Object {

        /// [Callback](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#Callback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::session::MediaController_Callback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionDestroyed](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onSessionDestroyed())
        pub fn onSessionDestroyed<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onSessionDestroyed", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onSessionDestroyed\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionEvent](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onSessionEvent(java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn onSessionEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onSessionEvent", .descriptor == "(Ljava/lang/String;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onSessionEvent\0", "(Ljava/lang/String;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPlaybackStateChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onPlaybackStateChanged(android.media.session.PlaybackState))
        ///
        /// Required features: "android-media-session-PlaybackState"
        #[cfg(any(feature = "all", all(feature = "android-media-session-PlaybackState")))]
        pub fn onPlaybackStateChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::PlaybackState>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onPlaybackStateChanged", .descriptor == "(Landroid/media/session/PlaybackState;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onPlaybackStateChanged\0", "(Landroid/media/session/PlaybackState;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onMetadataChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onMetadataChanged(android.media.MediaMetadata))
        ///
        /// Required features: "android-media-MediaMetadata"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata")))]
        pub fn onMetadataChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaMetadata>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onMetadataChanged", .descriptor == "(Landroid/media/MediaMetadata;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onMetadataChanged\0", "(Landroid/media/MediaMetadata;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onQueueChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onQueueChanged(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn onQueueChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onQueueChanged", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onQueueChanged\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onQueueTitleChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onQueueTitleChanged(java.lang.CharSequence))
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn onQueueTitleChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onQueueTitleChanged", .descriptor == "(Ljava/lang/CharSequence;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onQueueTitleChanged\0", "(Ljava/lang/CharSequence;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onExtrasChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onExtrasChanged(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn onExtrasChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onExtrasChanged", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onExtrasChanged\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAudioInfoChanged](https://developer.android.com/reference/android/media/session/MediaController.Callback.html#onAudioInfoChanged(android.media.session.MediaController.PlaybackInfo))
        ///
        /// Required features: "android-media-session-MediaController_PlaybackInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaController_PlaybackInfo")))]
        pub fn onAudioInfoChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaController_PlaybackInfo>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaController$Callback", java.flags == PUBLIC, .name == "onAudioInfoChanged", .descriptor == "(Landroid/media/session/MediaController$PlaybackInfo;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaController$Callback\0", "onAudioInfoChanged\0", "(Landroid/media/session/MediaController$PlaybackInfo;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
