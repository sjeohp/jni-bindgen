// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-ImageReader_OnImageAvailableListener"))]
__jni_bindgen! {
    /// public interface [ImageReader.OnImageAvailableListener](https://developer.android.com/reference/android/media/ImageReader.OnImageAvailableListener.html)
    ///
    /// Required feature: android-media-ImageReader_OnImageAvailableListener
    public interface ImageReader_OnImageAvailableListener ("android/media/ImageReader$OnImageAvailableListener") extends crate::java::lang::Object {

        /// [onImageAvailable](https://developer.android.com/reference/android/media/ImageReader.OnImageAvailableListener.html#onImageAvailable(android.media.ImageReader))
        ///
        /// Required features: "android-media-ImageReader"
        #[cfg(any(feature = "all", all(feature = "android-media-ImageReader")))]
        pub fn onImageAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::ImageReader>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/ImageReader$OnImageAvailableListener", java.flags == PUBLIC | ABSTRACT, .name == "onImageAvailable", .descriptor == "(Landroid/media/ImageReader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/ImageReader$OnImageAvailableListener\0", "onImageAvailable\0", "(Landroid/media/ImageReader;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
