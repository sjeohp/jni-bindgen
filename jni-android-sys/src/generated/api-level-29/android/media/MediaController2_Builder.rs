// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaController2_Builder"))]
__jni_bindgen! {
    /// public final class [MediaController2.Builder](https://developer.android.com/reference/android/media/MediaController2.Builder.html)
    ///
    /// Required feature: android-media-MediaController2_Builder
    public final class MediaController2_Builder ("android/media/MediaController2$Builder") extends crate::java::lang::Object {

        /// [Builder](https://developer.android.com/reference/android/media/MediaController2.Builder.html#Builder(android.content.Context,%20android.media.Session2Token))
        ///
        /// Required features: "android-content-Context", "android-media-Session2Token"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-media-Session2Token")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Token>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaController2_Builder>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$Builder", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/media/Session2Token;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$Builder\0", "<init>\0", "(Landroid/content/Context;Landroid/media/Session2Token;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setConnectionHints](https://developer.android.com/reference/android/media/MediaController2.Builder.html#setConnectionHints(android.os.Bundle))
        ///
        /// Required features: "android-media-MediaController2_Builder", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2_Builder", feature = "android-os-Bundle")))]
        pub fn setConnectionHints<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaController2_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$Builder", java.flags == PUBLIC, .name == "setConnectionHints", .descriptor == "(Landroid/os/Bundle;)Landroid/media/MediaController2$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$Builder\0", "setConnectionHints\0", "(Landroid/os/Bundle;)Landroid/media/MediaController2$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setControllerCallback](https://developer.android.com/reference/android/media/MediaController2.Builder.html#setControllerCallback(java.util.concurrent.Executor,%20android.media.MediaController2.ControllerCallback))
        ///
        /// Required features: "android-media-MediaController2_Builder", "android-media-MediaController2_ControllerCallback", "java-util-concurrent-Executor"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2_Builder", feature = "android-media-MediaController2_ControllerCallback", feature = "java-util-concurrent-Executor")))]
        pub fn setControllerCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::concurrent::Executor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2_ControllerCallback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaController2_Builder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$Builder", java.flags == PUBLIC, .name == "setControllerCallback", .descriptor == "(Ljava/util/concurrent/Executor;Landroid/media/MediaController2$ControllerCallback;)Landroid/media/MediaController2$Builder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$Builder\0", "setControllerCallback\0", "(Ljava/util/concurrent/Executor;Landroid/media/MediaController2$ControllerCallback;)Landroid/media/MediaController2$Builder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [build](https://developer.android.com/reference/android/media/MediaController2.Builder.html#build())
        ///
        /// Required features: "android-media-MediaController2"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2")))]
        pub fn build<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::MediaController2>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$Builder", java.flags == PUBLIC, .name == "build", .descriptor == "()Landroid/media/MediaController2;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$Builder\0", "build\0", "()Landroid/media/MediaController2;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
