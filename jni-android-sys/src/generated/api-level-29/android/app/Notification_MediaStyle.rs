// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-Notification_MediaStyle"))]
__jni_bindgen! {
    /// public class [Notification.MediaStyle](https://developer.android.com/reference/android/app/Notification.MediaStyle.html)
    ///
    /// Required feature: android-app-Notification_MediaStyle
    public class Notification_MediaStyle ("android/app/Notification$MediaStyle") extends crate::android::app::Notification_Style {

        /// [MediaStyle](https://developer.android.com/reference/android/app/Notification.MediaStyle.html#MediaStyle())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Notification_MediaStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$MediaStyle", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$MediaStyle\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [MediaStyle](https://developer.android.com/reference/android/app/Notification.MediaStyle.html#MediaStyle(android.app.Notification.Builder))
        ///
        /// Required features: "android-app-Notification_Builder"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_Builder")))]
        #[deprecated] pub fn new_Builder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Notification_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::Notification_MediaStyle>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$MediaStyle", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/app/Notification$Builder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$MediaStyle\0", "<init>\0", "(Landroid/app/Notification$Builder;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShowActionsInCompactView](https://developer.android.com/reference/android/app/Notification.MediaStyle.html#setShowActionsInCompactView(int...))
        ///
        /// Required features: "android-app-Notification_MediaStyle"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_MediaStyle")))]
        pub fn setShowActionsInCompactView<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_MediaStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$MediaStyle", java.flags == PUBLIC | VARARGS, .name == "setShowActionsInCompactView", .descriptor == "([I)Landroid/app/Notification$MediaStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$MediaStyle\0", "setShowActionsInCompactView\0", "([I)Landroid/app/Notification$MediaStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMediaSession](https://developer.android.com/reference/android/app/Notification.MediaStyle.html#setMediaSession(android.media.session.MediaSession.Token))
        ///
        /// Required features: "android-app-Notification_MediaStyle", "android-media-session-MediaSession_Token"
        #[cfg(any(feature = "all", all(feature = "android-app-Notification_MediaStyle", feature = "android-media-session-MediaSession_Token")))]
        pub fn setMediaSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSession_Token>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::Notification_MediaStyle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/Notification$MediaStyle", java.flags == PUBLIC, .name == "setMediaSession", .descriptor == "(Landroid/media/session/MediaSession$Token;)Landroid/app/Notification$MediaStyle;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/Notification$MediaStyle\0", "setMediaSession\0", "(Landroid/media/session/MediaSession$Token;)Landroid/app/Notification$MediaStyle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
