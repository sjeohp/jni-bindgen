// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-EnvironmentalReverb_OnParameterChangeListener"))]
__jni_bindgen! {
    /// public interface [EnvironmentalReverb.OnParameterChangeListener](https://developer.android.com/reference/android/media/audiofx/EnvironmentalReverb.OnParameterChangeListener.html)
    ///
    /// Required feature: android-media-audiofx-EnvironmentalReverb_OnParameterChangeListener
    public interface EnvironmentalReverb_OnParameterChangeListener ("android/media/audiofx/EnvironmentalReverb$OnParameterChangeListener") extends crate::java::lang::Object {

        /// [onParameterChange](https://developer.android.com/reference/android/media/audiofx/EnvironmentalReverb.OnParameterChangeListener.html#onParameterChange(android.media.audiofx.EnvironmentalReverb,%20int,%20int,%20int))
        ///
        /// Required features: "android-media-audiofx-EnvironmentalReverb"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-EnvironmentalReverb")))]
        pub fn onParameterChange<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::EnvironmentalReverb>>, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/EnvironmentalReverb$OnParameterChangeListener", java.flags == PUBLIC | ABSTRACT, .name == "onParameterChange", .descriptor == "(Landroid/media/audiofx/EnvironmentalReverb;III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/EnvironmentalReverb$OnParameterChangeListener\0", "onParameterChange\0", "(Landroid/media/audiofx/EnvironmentalReverb;III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
