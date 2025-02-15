// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaMetadata_Builder"))]
__jni_bindgen! {
    /// public final class [MediaMetadata.Builder](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html)
    ///
    /// Required feature: android-media-MediaMetadata_Builder
    public final class MediaMetadata_Builder ("android/media/MediaMetadata$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#Builder())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Builder](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#Builder(android.media.MediaMetadata))
        ///
        /// Required features: "android-media-MediaMetadata"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata")))]
        pub fn new_MediaMetadata<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaMetadata>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/media/MediaMetadata;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "<init>\0", "(Landroid/media/MediaMetadata;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putText](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#putText(java.lang.String,%20java.lang.CharSequence))
        ///
        /// Required features: "android-media-MediaMetadata_Builder", "java-lang-CharSequence", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata_Builder", feature = "java-lang-CharSequence", feature = "java-lang-String")))]
        pub fn putText<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::CharSequence>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "putText", .descriptor == "(Ljava/lang/String;Ljava/lang/CharSequence;)Landroid/media/MediaMetadata$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "putText\0", "(Ljava/lang/String;Ljava/lang/CharSequence;)Landroid/media/MediaMetadata$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putString](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#putString(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "android-media-MediaMetadata_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata_Builder", feature = "java-lang-String")))]
        pub fn putString<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "putString", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Landroid/media/MediaMetadata$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "putString\0", "(Ljava/lang/String;Ljava/lang/String;)Landroid/media/MediaMetadata$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putLong](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#putLong(java.lang.String,%20long))
        ///
        /// Required features: "android-media-MediaMetadata_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata_Builder", feature = "java-lang-String")))]
        pub fn putLong<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "putLong", .descriptor == "(Ljava/lang/String;J)Landroid/media/MediaMetadata$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "putLong\0", "(Ljava/lang/String;J)Landroid/media/MediaMetadata$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putRating](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#putRating(java.lang.String,%20android.media.Rating))
        ///
        /// Required features: "android-media-MediaMetadata_Builder", "android-media-Rating", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata_Builder", feature = "android-media-Rating", feature = "java-lang-String")))]
        pub fn putRating<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Rating>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "putRating", .descriptor == "(Ljava/lang/String;Landroid/media/Rating;)Landroid/media/MediaMetadata$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "putRating\0", "(Ljava/lang/String;Landroid/media/Rating;)Landroid/media/MediaMetadata$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [putBitmap](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#putBitmap(java.lang.String,%20android.graphics.Bitmap))
        ///
        /// Required features: "android-graphics-Bitmap", "android-media-MediaMetadata_Builder", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-graphics-Bitmap", feature = "android-media-MediaMetadata_Builder", feature = "java-lang-String")))]
        pub fn putBitmap<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::graphics::Bitmap>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "putBitmap", .descriptor == "(Ljava/lang/String;Landroid/graphics/Bitmap;)Landroid/media/MediaMetadata$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "putBitmap\0", "(Ljava/lang/String;Landroid/graphics/Bitmap;)Landroid/media/MediaMetadata$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/media/MediaMetadata.Builder.html#build())
        ///
        /// Required features: "android-media-MediaMetadata"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaMetadata")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaMetadata>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaMetadata$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/media/MediaMetadata;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaMetadata$Builder\0", "build\0", "()Landroid/media/MediaMetadata;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
