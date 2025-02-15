// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-audiofx-Visualizer_OnDataCaptureListener"))]
__jni_bindgen! {
    /// public interface [Visualizer.OnDataCaptureListener](https://developer.android.com/reference/android/media/audiofx/Visualizer.OnDataCaptureListener.html)
    ///
    /// Required feature: android-media-audiofx-Visualizer_OnDataCaptureListener
    public interface Visualizer_OnDataCaptureListener ("android/media/audiofx/Visualizer$OnDataCaptureListener") extends crate::java::lang::Object {

        /// [onWaveFormDataCapture](https://developer.android.com/reference/android/media/audiofx/Visualizer.OnDataCaptureListener.html#onWaveFormDataCapture(android.media.audiofx.Visualizer,%20byte%5B%5D,%20int))
        ///
        /// Required features: "android-media-audiofx-Visualizer"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-Visualizer")))]
        pub fn onWaveFormDataCapture<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::Visualizer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer$OnDataCaptureListener", java.flags == PUBLIC | ABSTRACT, .name == "onWaveFormDataCapture", .descriptor == "(Landroid/media/audiofx/Visualizer;[BI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer$OnDataCaptureListener\0", "onWaveFormDataCapture\0", "(Landroid/media/audiofx/Visualizer;[BI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFftDataCapture](https://developer.android.com/reference/android/media/audiofx/Visualizer.OnDataCaptureListener.html#onFftDataCapture(android.media.audiofx.Visualizer,%20byte%5B%5D,%20int))
        ///
        /// Required features: "android-media-audiofx-Visualizer"
        #[cfg(any(feature = "all", all(feature = "android-media-audiofx-Visualizer")))]
        pub fn onFftDataCapture<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::audiofx::Visualizer>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/audiofx/Visualizer$OnDataCaptureListener", java.flags == PUBLIC | ABSTRACT, .name == "onFftDataCapture", .descriptor == "(Landroid/media/audiofx/Visualizer;[BI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/audiofx/Visualizer$OnDataCaptureListener\0", "onFftDataCapture\0", "(Landroid/media/audiofx/Visualizer;[BI)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
