// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-tv-TvTrackInfo"))]
__jni_bindgen! {
    /// public final class [TvTrackInfo](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html)
    ///
    /// Required feature: android-media-tv-TvTrackInfo
    public final class TvTrackInfo ("android/media/tv/TvTrackInfo") extends crate::java::lang::Object, implements crate::android::os::Parcelable {

        // // Not emitting: Non-public method
        // /// [TvTrackInfo](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#TvTrackInfo(android.os.Parcel))
        // ///
        // /// Required features: "android-os-Parcel"
        // #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::tv::TvTrackInfo>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/tv/TvTrackInfo", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/os/Parcel;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "<init>\0", "(Landroid/os/Parcel;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getType](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getType())
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getId](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getId())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getId<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getId", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getId\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLanguage](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getLanguage())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getLanguage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getLanguage", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getLanguage\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDescription](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getDescription())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        pub fn getDescription<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getDescription", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getDescription\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAudioChannelCount](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getAudioChannelCount())
        pub fn getAudioChannelCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getAudioChannelCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getAudioChannelCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAudioSampleRate](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getAudioSampleRate())
        pub fn getAudioSampleRate<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getAudioSampleRate", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getAudioSampleRate\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVideoWidth](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getVideoWidth())
        pub fn getVideoWidth<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getVideoWidth", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getVideoWidth\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVideoHeight](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getVideoHeight())
        pub fn getVideoHeight<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getVideoHeight", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getVideoHeight\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVideoFrameRate](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getVideoFrameRate())
        pub fn getVideoFrameRate<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getVideoFrameRate", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getVideoFrameRate\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVideoPixelAspectRatio](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getVideoPixelAspectRatio())
        pub fn getVideoPixelAspectRatio<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getVideoPixelAspectRatio", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getVideoPixelAspectRatio\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVideoActiveFormatDescription](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getVideoActiveFormatDescription())
        pub fn getVideoActiveFormatDescription<'env>(&'env self) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getVideoActiveFormatDescription", .descriptor == "()B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getVideoActiveFormatDescription\0", "()B\0");
                __jni_env.call_byte_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtra](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#getExtra())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getExtra<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "getExtra", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "getExtra\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [describeContents](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#describeContents())
        pub fn describeContents<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "describeContents", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "describeContents\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [writeToParcel](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#writeToParcel(android.os.Parcel,%20int))
        ///
        /// Required features: "android-os-Parcel"
        #[cfg(any(feature = "all", all(feature = "android-os-Parcel")))]
        pub fn writeToParcel<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Parcel>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "writeToParcel", .descriptor == "(Landroid/os/Parcel;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "writeToParcel\0", "(Landroid/os/Parcel;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/tv/TvTrackInfo", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/tv/TvTrackInfo\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [CREATOR](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#CREATOR)
        ///
        /// Required feature: android-os-Parcelable_Creator
        #[cfg(any(feature = "all", feature = "android-os-Parcelable_Creator"))]
        pub fn CREATOR<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Parcelable_Creator>> {
            unsafe {
                let (class, field) = env.require_class_static_field("android/media/tv/TvTrackInfo\0", "CREATOR\0", "Landroid/os/Parcelable$Creator;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// public static final [TYPE_AUDIO](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#TYPE_AUDIO)
        pub const TYPE_AUDIO : i32 = 0;

        /// public static final [TYPE_SUBTITLE](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#TYPE_SUBTITLE)
        pub const TYPE_SUBTITLE : i32 = 2;

        /// public static final [TYPE_VIDEO](https://developer.android.com/reference/android/media/tv/TvTrackInfo.html#TYPE_VIDEO)
        pub const TYPE_VIDEO : i32 = 1;
    }
}
