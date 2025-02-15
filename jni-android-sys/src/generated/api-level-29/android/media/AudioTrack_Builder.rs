// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-AudioTrack_Builder"))]
__jni_bindgen! {
    /// public class [AudioTrack.Builder](https://developer.android.com/reference/android/media/AudioTrack.Builder.html)
    ///
    /// Required feature: android-media-AudioTrack_Builder
    public class AudioTrack_Builder ("android/media/AudioTrack$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAudioAttributes](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setAudioAttributes(android.media.AudioAttributes))
        ///
        /// Required features: "android-media-AudioAttributes", "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioAttributes", feature = "android-media-AudioTrack_Builder")))]
        pub fn setAudioAttributes<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::AudioAttributes>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setAudioAttributes", .descriptor == "(Landroid/media/AudioAttributes;)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setAudioAttributes\0", "(Landroid/media/AudioAttributes;)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAudioFormat](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setAudioFormat(android.media.AudioFormat))
        ///
        /// Required features: "android-media-AudioFormat", "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioFormat", feature = "android-media-AudioTrack_Builder")))]
        pub fn setAudioFormat<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::AudioFormat>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setAudioFormat", .descriptor == "(Landroid/media/AudioFormat;)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setAudioFormat\0", "(Landroid/media/AudioFormat;)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBufferSizeInBytes](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setBufferSizeInBytes(int))
        ///
        /// Required features: "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack_Builder")))]
        pub fn setBufferSizeInBytes<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setBufferSizeInBytes", .descriptor == "(I)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setBufferSizeInBytes\0", "(I)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTransferMode](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setTransferMode(int))
        ///
        /// Required features: "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack_Builder")))]
        pub fn setTransferMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setTransferMode", .descriptor == "(I)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setTransferMode\0", "(I)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSessionId](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setSessionId(int))
        ///
        /// Required features: "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack_Builder")))]
        pub fn setSessionId<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setSessionId", .descriptor == "(I)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setSessionId\0", "(I)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPerformanceMode](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setPerformanceMode(int))
        ///
        /// Required features: "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack_Builder")))]
        pub fn setPerformanceMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setPerformanceMode", .descriptor == "(I)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setPerformanceMode\0", "(I)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOffloadedPlayback](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#setOffloadedPlayback(boolean))
        ///
        /// Required features: "android-media-AudioTrack_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack_Builder")))]
        pub fn setOffloadedPlayback<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "setOffloadedPlayback", .descriptor == "(Z)Landroid/media/AudioTrack$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "setOffloadedPlayback\0", "(Z)Landroid/media/AudioTrack$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/media/AudioTrack.Builder.html#build())
        ///
        /// Required features: "android-media-AudioTrack"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::AudioTrack>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioTrack$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/media/AudioTrack;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioTrack$Builder\0", "build\0", "()Landroid/media/AudioTrack;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
