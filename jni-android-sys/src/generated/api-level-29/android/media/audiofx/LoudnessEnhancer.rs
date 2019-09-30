// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-LoudnessEnhancer"))]
__jni_bindgen! {
    /// public class [LoudnessEnhancer](https://developer.android.com/reference/android/media/audiofx/LoudnessEnhancer.html)
    ///
    /// Required feature: android-media-audiofx-LoudnessEnhancer
    public class LoudnessEnhancer ("android/media/audiofx/LoudnessEnhancer") extends crate::android::media::audiofx::AudioEffect {

        /// [LoudnessEnhancer](https://developer.android.com/reference/android/media/audiofx/LoudnessEnhancer.html#LoudnessEnhancer(int))
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::audiofx::LoudnessEnhancer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/LoudnessEnhancer", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/LoudnessEnhancer\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTargetGain](https://developer.android.com/reference/android/media/audiofx/LoudnessEnhancer.html#setTargetGain(int))
        pub fn setTargetGain<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/LoudnessEnhancer", java.flags == PUBLIC, .name == "setTargetGain", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/LoudnessEnhancer\0", "setTargetGain\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTargetGain](https://developer.android.com/reference/android/media/audiofx/LoudnessEnhancer.html#getTargetGain())
        pub fn getTargetGain<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/LoudnessEnhancer", java.flags == PUBLIC, .name == "getTargetGain", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/LoudnessEnhancer\0", "getTargetGain\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [PARAM_TARGET_GAIN_MB](https://developer.android.com/reference/android/media/audiofx/LoudnessEnhancer.html#PARAM_TARGET_GAIN_MB)
        pub const PARAM_TARGET_GAIN_MB : i32 = 0;
    }
}
