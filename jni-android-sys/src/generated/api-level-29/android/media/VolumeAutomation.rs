// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-VolumeAutomation"))]
__jni_bindgen! {
    /// public interface [VolumeAutomation](https://developer.android.com/reference/android/media/VolumeAutomation.html)
    ///
    /// Required feature: android-media-VolumeAutomation
    public interface VolumeAutomation ("android/media/VolumeAutomation") extends crate::java::lang::Object {

        /// [createVolumeShaper](https://developer.android.com/reference/android/media/VolumeAutomation.html#createVolumeShaper(android.media.VolumeShaper.Configuration))
        ///
        /// Required features: "android-media-VolumeShaper", "android-media-VolumeShaper_Configuration"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper", feature = "android-media-VolumeShaper_Configuration")))]
        pub fn createVolumeShaper<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::VolumeShaper_Configuration>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeAutomation", java.flags == PUBLIC | ABSTRACT, .name == "createVolumeShaper", .descriptor == "(Landroid/media/VolumeShaper$Configuration;)Landroid/media/VolumeShaper;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeAutomation\0", "createVolumeShaper\0", "(Landroid/media/VolumeShaper$Configuration;)Landroid/media/VolumeShaper;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
