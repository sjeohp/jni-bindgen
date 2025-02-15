// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-AudioPresentation"))]
__jni_bindgen! {
    /// public final class [AudioPresentation](https://developer.android.com/reference/android/media/AudioPresentation.html)
    ///
    /// Required feature: android-media-AudioPresentation
    public final class AudioPresentation ("android/media/AudioPresentation") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [AudioPresentation](https://developer.android.com/reference/android/media/AudioPresentation.html#AudioPresentation(int,%20int,%20android.icu.util.ULocale,%20int,%20boolean,%20boolean,%20boolean,%20java.util.Map))
        // ///
        // /// Required features: "android-icu-util-ULocale", "java-util-Map"
        // #[cfg(any(feature = "all", all(feature = "android-icu-util-ULocale", feature = "java-util-Map")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::icu::util::ULocale>>, arg3: i32, arg4: bool, arg5: bool, arg6: bool, arg7: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Map>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::AudioPresentation>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/AudioPresentation", java.flags == (empty), .name == "<init>", .descriptor == "(IILandroid/icu/util/ULocale;IZZZLjava/util/Map;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5), __jni_bindgen::AsJValue::as_jvalue(&arg6), __jni_bindgen::AsJValue::as_jvalue(&arg7.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "<init>\0", "(IILandroid/icu/util/ULocale;IZZZLjava/util/Map;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getPresentationId](https://developer.android.com/reference/android/media/AudioPresentation.html#getPresentationId())
        pub fn getPresentationId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "getPresentationId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "getPresentationId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProgramId](https://developer.android.com/reference/android/media/AudioPresentation.html#getProgramId())
        pub fn getProgramId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "getProgramId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "getProgramId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLabels](https://developer.android.com/reference/android/media/AudioPresentation.html#getLabels())
        ///
        /// Required features: "java-util-Map"
        #[cfg(any(feature = "all", all(feature = "java-util-Map")))]
        pub fn getLabels<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Map>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "getLabels", .descriptor == "()Ljava/util/Map;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "getLabels\0", "()Ljava/util/Map;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocale](https://developer.android.com/reference/android/media/AudioPresentation.html#getLocale())
        ///
        /// Required features: "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-util-Locale")))]
        pub fn getLocale<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Locale>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "getLocale", .descriptor == "()Ljava/util/Locale;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "getLocale\0", "()Ljava/util/Locale;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMasteringIndication](https://developer.android.com/reference/android/media/AudioPresentation.html#getMasteringIndication())
        pub fn getMasteringIndication<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "getMasteringIndication", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "getMasteringIndication\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasAudioDescription](https://developer.android.com/reference/android/media/AudioPresentation.html#hasAudioDescription())
        pub fn hasAudioDescription<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "hasAudioDescription", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "hasAudioDescription\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasSpokenSubtitles](https://developer.android.com/reference/android/media/AudioPresentation.html#hasSpokenSubtitles())
        pub fn hasSpokenSubtitles<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "hasSpokenSubtitles", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "hasSpokenSubtitles\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasDialogueEnhancement](https://developer.android.com/reference/android/media/AudioPresentation.html#hasDialogueEnhancement())
        pub fn hasDialogueEnhancement<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "hasDialogueEnhancement", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "hasDialogueEnhancement\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/android/media/AudioPresentation.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/android/media/AudioPresentation.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/android/media/AudioPresentation.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/AudioPresentation", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/AudioPresentation\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [MASTERED_FOR_3D](https://developer.android.com/reference/android/media/AudioPresentation.html#MASTERED_FOR_3D)
        pub const MASTERED_FOR_3D : i32 = 3;

        /// public static final [MASTERED_FOR_HEADPHONE](https://developer.android.com/reference/android/media/AudioPresentation.html#MASTERED_FOR_HEADPHONE)
        pub const MASTERED_FOR_HEADPHONE : i32 = 4;

        /// public static final [MASTERED_FOR_STEREO](https://developer.android.com/reference/android/media/AudioPresentation.html#MASTERED_FOR_STEREO)
        pub const MASTERED_FOR_STEREO : i32 = 1;

        /// public static final [MASTERED_FOR_SURROUND](https://developer.android.com/reference/android/media/AudioPresentation.html#MASTERED_FOR_SURROUND)
        pub const MASTERED_FOR_SURROUND : i32 = 2;

        /// public static final [MASTERING_NOT_INDICATED](https://developer.android.com/reference/android/media/AudioPresentation.html#MASTERING_NOT_INDICATED)
        pub const MASTERING_NOT_INDICATED : i32 = 0;
    }
}
