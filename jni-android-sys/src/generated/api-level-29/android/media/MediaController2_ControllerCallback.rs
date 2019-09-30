// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-MediaController2_ControllerCallback"))]
__jni_bindgen! {
    /// public class [MediaController2.ControllerCallback](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html)
    ///
    /// Required feature: android-media-MediaController2_ControllerCallback
    public class MediaController2_ControllerCallback ("android/media/MediaController2$ControllerCallback") extends crate::java::lang::Object {

        /// [ControllerCallback](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#ControllerCallback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::MediaController2_ControllerCallback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnected](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#onConnected(android.media.MediaController2,%20android.media.Session2CommandGroup))
        ///
        /// Required features: "android-media-MediaController2", "android-media-Session2CommandGroup"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2", feature = "android-media-Session2CommandGroup")))]
        pub fn onConnected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2CommandGroup>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "onConnected", .descriptor == "(Landroid/media/MediaController2;Landroid/media/Session2CommandGroup;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "onConnected\0", "(Landroid/media/MediaController2;Landroid/media/Session2CommandGroup;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnected](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#onDisconnected(android.media.MediaController2))
        ///
        /// Required features: "android-media-MediaController2"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2")))]
        pub fn onDisconnected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "onDisconnected", .descriptor == "(Landroid/media/MediaController2;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "onDisconnected\0", "(Landroid/media/MediaController2;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPlaybackActiveChanged](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#onPlaybackActiveChanged(android.media.MediaController2,%20boolean))
        ///
        /// Required features: "android-media-MediaController2"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2")))]
        pub fn onPlaybackActiveChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2>>, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "onPlaybackActiveChanged", .descriptor == "(Landroid/media/MediaController2;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "onPlaybackActiveChanged\0", "(Landroid/media/MediaController2;Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionCommand](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#onSessionCommand(android.media.MediaController2,%20android.media.Session2Command,%20android.os.Bundle))
        ///
        /// Required features: "android-media-MediaController2", "android-media-Session2Command", "android-media-Session2Command_Result", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2", feature = "android-media-Session2Command", feature = "android-media-Session2Command_Result", feature = "android-os-Bundle")))]
        pub fn onSessionCommand<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Command>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::Session2Command_Result>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "onSessionCommand", .descriptor == "(Landroid/media/MediaController2;Landroid/media/Session2Command;Landroid/os/Bundle;)Landroid/media/Session2Command$Result;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "onSessionCommand\0", "(Landroid/media/MediaController2;Landroid/media/Session2Command;Landroid/os/Bundle;)Landroid/media/Session2Command$Result;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCommandResult](https://developer.android.com/reference/android/media/MediaController2.ControllerCallback.html#onCommandResult(android.media.MediaController2,%20java.lang.Object,%20android.media.Session2Command,%20android.media.Session2Command.Result))
        ///
        /// Required features: "android-media-MediaController2", "android-media-Session2Command", "android-media-Session2Command_Result", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "android-media-MediaController2", feature = "android-media-Session2Command", feature = "android-media-Session2Command_Result", feature = "java-lang-Object")))]
        pub fn onCommandResult<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::MediaController2>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Command>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Command_Result>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/MediaController2$ControllerCallback", java.flags == PUBLIC, .name == "onCommandResult", .descriptor == "(Landroid/media/MediaController2;Ljava/lang/Object;Landroid/media/Session2Command;Landroid/media/Session2Command$Result;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/MediaController2$ControllerCallback\0", "onCommandResult\0", "(Landroid/media/MediaController2;Ljava/lang/Object;Landroid/media/Session2Command;Landroid/media/Session2Command$Result;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
