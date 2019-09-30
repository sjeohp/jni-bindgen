// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-PresetReverb"))]
__jni_bindgen! {
    /// public class [PresetReverb](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html)
    ///
    /// Required feature: android-media-audiofx-PresetReverb
    public class PresetReverb ("android/media/audiofx/PresetReverb") extends crate::android::media::audiofx::AudioEffect {

        /// [PresetReverb](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PresetReverb(int,%20int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::audiofx::PresetReverb>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "<init>", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "<init>\0", "(II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPreset](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#setPreset(short))
        pub fn setPreset<'env>(&'env self, arg0: i16) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "setPreset", .descriptor == "(S)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "setPreset\0", "(S)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPreset](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#getPreset())
        pub fn getPreset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i16, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "getPreset", .descriptor == "()S"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "getPreset\0", "()S\0");
                __jni_env.call_short_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setParameterListener](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#setParameterListener(android.media.audiofx.PresetReverb.OnParameterChangeListener))
        ///
        /// Required features: "android-media-audiofx-PresetReverb_OnParameterChangeListener"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-PresetReverb_OnParameterChangeListener")))]
        pub fn setParameterListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::PresetReverb_OnParameterChangeListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "setParameterListener", .descriptor == "(Landroid/media/audiofx/PresetReverb$OnParameterChangeListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "setParameterListener\0", "(Landroid/media/audiofx/PresetReverb$OnParameterChangeListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProperties](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#getProperties())
        ///
        /// Required features: "android-media-audiofx-PresetReverb_Settings"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-PresetReverb_Settings")))]
        pub fn getProperties<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::audiofx::PresetReverb_Settings>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "getProperties", .descriptor == "()Landroid/media/audiofx/PresetReverb$Settings;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "getProperties\0", "()Landroid/media/audiofx/PresetReverb$Settings;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setProperties](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#setProperties(android.media.audiofx.PresetReverb.Settings))
        ///
        /// Required features: "android-media-audiofx-PresetReverb_Settings"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-PresetReverb_Settings")))]
        pub fn setProperties<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::PresetReverb_Settings>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/PresetReverb", java.flags == PUBLIC, .name == "setProperties", .descriptor == "(Landroid/media/audiofx/PresetReverb$Settings;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/PresetReverb\0", "setProperties\0", "(Landroid/media/audiofx/PresetReverb$Settings;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [PARAM_PRESET](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PARAM_PRESET)
        pub const PARAM_PRESET : i32 = 0;

        /// public static final [PRESET_LARGEHALL](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_LARGEHALL)
        pub const PRESET_LARGEHALL : i16 = 5;

        /// public static final [PRESET_LARGEROOM](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_LARGEROOM)
        pub const PRESET_LARGEROOM : i16 = 3;

        /// public static final [PRESET_MEDIUMHALL](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_MEDIUMHALL)
        pub const PRESET_MEDIUMHALL : i16 = 4;

        /// public static final [PRESET_MEDIUMROOM](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_MEDIUMROOM)
        pub const PRESET_MEDIUMROOM : i16 = 2;

        /// public static final [PRESET_NONE](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_NONE)
        pub const PRESET_NONE : i16 = 0;

        /// public static final [PRESET_PLATE](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_PLATE)
        pub const PRESET_PLATE : i16 = 6;

        /// public static final [PRESET_SMALLROOM](https://developer.android.com/reference/android/media/audiofx/PresetReverb.html#PRESET_SMALLROOM)
        pub const PRESET_SMALLROOM : i16 = 1;
    }
}
