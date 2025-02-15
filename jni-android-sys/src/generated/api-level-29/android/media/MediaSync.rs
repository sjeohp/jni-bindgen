// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaSync"))]
__jni_bindgen! {
    /// public final class [MediaSync](https://developer.android.com/reference/android/media/MediaSync.html)
    ///
    /// Required feature: android-media-MediaSync
    public final class MediaSync ("android/media/MediaSync") extends crate::java::lang::Object {

        /// [MediaSync](https://developer.android.com/reference/android/media/MediaSync.html#MediaSync())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaSync>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/media/MediaSync.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/MediaSync", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [release](https://developer.android.com/reference/android/media/MediaSync.html#release())
        pub fn release<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "release", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "release\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCallback](https://developer.android.com/reference/android/media/MediaSync.html#setCallback(android.media.MediaSync.Callback,%20android.os.Handler))
        ///
        /// Required features: "android-media-MediaSync_Callback", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaSync_Callback", feature = "android-os-Handler")))]
        pub fn setCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaSync_Callback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setCallback", .descriptor == "(Landroid/media/MediaSync$Callback;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setCallback\0", "(Landroid/media/MediaSync$Callback;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOnErrorListener](https://developer.android.com/reference/android/media/MediaSync.html#setOnErrorListener(android.media.MediaSync.OnErrorListener,%20android.os.Handler))
        ///
        /// Required features: "android-media-MediaSync_OnErrorListener", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaSync_OnErrorListener", feature = "android-os-Handler")))]
        pub fn setOnErrorListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaSync_OnErrorListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setOnErrorListener", .descriptor == "(Landroid/media/MediaSync$OnErrorListener;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setOnErrorListener\0", "(Landroid/media/MediaSync$OnErrorListener;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSurface](https://developer.android.com/reference/android/media/MediaSync.html#setSurface(android.view.Surface))
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn setSurface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::view::Surface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setSurface", .descriptor == "(Landroid/view/Surface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setSurface\0", "(Landroid/view/Surface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAudioTrack](https://developer.android.com/reference/android/media/MediaSync.html#setAudioTrack(android.media.AudioTrack))
        ///
        /// Required features: "android-media-AudioTrack"
        #[cfg(any(feature = "all", all(feature = "android-media-AudioTrack")))]
        pub fn setAudioTrack<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::AudioTrack>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setAudioTrack", .descriptor == "(Landroid/media/AudioTrack;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setAudioTrack\0", "(Landroid/media/AudioTrack;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createInputSurface](https://developer.android.com/reference/android/media/MediaSync.html#createInputSurface())
        ///
        /// Required features: "android-view-Surface"
        #[cfg(any(feature = "all", all(feature = "android-view-Surface")))]
        pub fn createInputSurface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::view::Surface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC | NATIVE, .name == "createInputSurface", .descriptor == "()Landroid/view/Surface;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "createInputSurface\0", "()Landroid/view/Surface;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPlaybackParams](https://developer.android.com/reference/android/media/MediaSync.html#setPlaybackParams(android.media.PlaybackParams))
        ///
        /// Required features: "android-media-PlaybackParams"
        #[cfg(any(feature = "all", all(feature = "android-media-PlaybackParams")))]
        pub fn setPlaybackParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::PlaybackParams>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setPlaybackParams", .descriptor == "(Landroid/media/PlaybackParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setPlaybackParams\0", "(Landroid/media/PlaybackParams;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPlaybackParams](https://developer.android.com/reference/android/media/MediaSync.html#getPlaybackParams())
        ///
        /// Required features: "android-media-PlaybackParams"
        #[cfg(any(feature = "all", all(feature = "android-media-PlaybackParams")))]
        pub fn getPlaybackParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::PlaybackParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC | NATIVE, .name == "getPlaybackParams", .descriptor == "()Landroid/media/PlaybackParams;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "getPlaybackParams\0", "()Landroid/media/PlaybackParams;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSyncParams](https://developer.android.com/reference/android/media/MediaSync.html#setSyncParams(android.media.SyncParams))
        ///
        /// Required features: "android-media-SyncParams"
        #[cfg(any(feature = "all", all(feature = "android-media-SyncParams")))]
        pub fn setSyncParams<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::SyncParams>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "setSyncParams", .descriptor == "(Landroid/media/SyncParams;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "setSyncParams\0", "(Landroid/media/SyncParams;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSyncParams](https://developer.android.com/reference/android/media/MediaSync.html#getSyncParams())
        ///
        /// Required features: "android-media-SyncParams"
        #[cfg(any(feature = "all", all(feature = "android-media-SyncParams")))]
        pub fn getSyncParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::SyncParams>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC | NATIVE, .name == "getSyncParams", .descriptor == "()Landroid/media/SyncParams;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "getSyncParams\0", "()Landroid/media/SyncParams;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [flush](https://developer.android.com/reference/android/media/MediaSync.html#flush())
        pub fn flush<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "flush", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "flush\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimestamp](https://developer.android.com/reference/android/media/MediaSync.html#getTimestamp())
        ///
        /// Required features: "android-media-MediaTimestamp"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaTimestamp")))]
        pub fn getTimestamp<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaTimestamp>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "getTimestamp", .descriptor == "()Landroid/media/MediaTimestamp;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "getTimestamp\0", "()Landroid/media/MediaTimestamp;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [queueAudio](https://developer.android.com/reference/android/media/MediaSync.html#queueAudio(java.nio.ByteBuffer,%20int,%20long))
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn queueAudio<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>, arg1: i32, arg2: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaSync", java.flags == PUBLIC, .name == "queueAudio", .descriptor == "(Ljava/nio/ByteBuffer;IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaSync\0", "queueAudio\0", "(Ljava/nio/ByteBuffer;IJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [MEDIASYNC_ERROR_AUDIOTRACK_FAIL](https://developer.android.com/reference/android/media/MediaSync.html#MEDIASYNC_ERROR_AUDIOTRACK_FAIL)
        pub const MEDIASYNC_ERROR_AUDIOTRACK_FAIL : i32 = 1;

        /// public static final [MEDIASYNC_ERROR_SURFACE_FAIL](https://developer.android.com/reference/android/media/MediaSync.html#MEDIASYNC_ERROR_SURFACE_FAIL)
        pub const MEDIASYNC_ERROR_SURFACE_FAIL : i32 = 2;
    }
}
