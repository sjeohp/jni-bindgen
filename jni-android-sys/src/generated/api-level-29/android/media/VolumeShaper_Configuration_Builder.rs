// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-VolumeShaper_Configuration_Builder"))]
__jni_bindgen! {
    /// public final class [VolumeShaper.Configuration.Builder](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html)
    ///
    /// Required feature: android-media-VolumeShaper_Configuration_Builder
    public final class VolumeShaper_Configuration_Builder ("android/media/VolumeShaper$Configuration$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Builder](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#Builder(android.media.VolumeShaper.Configuration))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration")))]
        pub fn new_Configuration<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::VolumeShaper_Configuration>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/media/VolumeShaper$Configuration;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "<init>\0", "(Landroid/media/VolumeShaper$Configuration;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInterpolatorType](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#setInterpolatorType(int))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn setInterpolatorType<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "setInterpolatorType", .descriptor == "(I)Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "setInterpolatorType\0", "(I)Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDuration](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#setDuration(long))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn setDuration<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "setDuration", .descriptor == "(J)Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "setDuration\0", "(J)Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCurve](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#setCurve(float%5B%5D,%20float%5B%5D))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn setCurve<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "setCurve", .descriptor == "([F[F)Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "setCurve\0", "([F[F)Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reflectTimes](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#reflectTimes())
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn reflectTimes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "reflectTimes", .descriptor == "()Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "reflectTimes\0", "()Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [invertVolumes](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#invertVolumes())
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn invertVolumes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "invertVolumes", .descriptor == "()Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "invertVolumes\0", "()Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [scaleToEndVolume](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#scaleToEndVolume(float))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn scaleToEndVolume<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "scaleToEndVolume", .descriptor == "(F)Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "scaleToEndVolume\0", "(F)Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [scaleToStartVolume](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#scaleToStartVolume(float))
        ///
        /// Required features: "android-media-VolumeShaper_Configuration_Builder"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration_Builder")))]
        pub fn scaleToStartVolume<'env>(&'env self, arg0: f32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "scaleToStartVolume", .descriptor == "(F)Landroid/media/VolumeShaper$Configuration$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "scaleToStartVolume\0", "(F)Landroid/media/VolumeShaper$Configuration$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/media/VolumeShaper.Configuration.Builder.html#build())
        ///
        /// Required features: "android-media-VolumeShaper_Configuration"
        #[cfg(any(feature = "all", all(feature = "android-media-VolumeShaper_Configuration")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::VolumeShaper_Configuration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/VolumeShaper$Configuration$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/media/VolumeShaper$Configuration;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/VolumeShaper$Configuration$Builder\0", "build\0", "()Landroid/media/VolumeShaper$Configuration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
