// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-session-MediaSessionManager"))]
__jni_bindgen! {
    /// public final class [MediaSessionManager](https://developer.android.com/reference/android/media/session/MediaSessionManager.html)
    ///
    /// Required feature: android-media-session-MediaSessionManager
    public final class MediaSessionManager ("android/media/session/MediaSessionManager") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MediaSessionManager](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#MediaSessionManager(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::session::MediaSessionManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/session/MediaSessionManager", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [notifySession2Created](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#notifySession2Created(android.media.Session2Token))
        ///
        /// Required features: "android-media-Session2Token"
        #[cfg(any(feature = "all", all(feature = "android-media-Session2Token")))]
        pub fn notifySession2Created<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::Session2Token>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "notifySession2Created", .descriptor == "(Landroid/media/Session2Token;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "notifySession2Created\0", "(Landroid/media/Session2Token;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getActiveSessions](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#getActiveSessions(android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "java-util-List")))]
        pub fn getActiveSessions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "getActiveSessions", .descriptor == "(Landroid/content/ComponentName;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "getActiveSessions\0", "(Landroid/content/ComponentName;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSession2Tokens](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#getSession2Tokens())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getSession2Tokens<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "getSession2Tokens", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "getSession2Tokens\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnActiveSessionsChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#addOnActiveSessionsChangedListener(android.media.session.MediaSessionManager.OnActiveSessionsChangedListener,%20android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName", "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener")))]
        pub fn addOnActiveSessionsChangedListener_OnActiveSessionsChangedListener_ComponentName<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnActiveSessionsChangedListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "addOnActiveSessionsChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;Landroid/content/ComponentName;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "addOnActiveSessionsChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;Landroid/content/ComponentName;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnActiveSessionsChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#addOnActiveSessionsChangedListener(android.media.session.MediaSessionManager.OnActiveSessionsChangedListener,%20android.content.ComponentName,%20android.os.Handler))
        ///
        /// Required features: "android-content-ComponentName", "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener", feature = "android-os-Handler")))]
        pub fn addOnActiveSessionsChangedListener_OnActiveSessionsChangedListener_ComponentName_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnActiveSessionsChangedListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "addOnActiveSessionsChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;Landroid/content/ComponentName;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "addOnActiveSessionsChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;Landroid/content/ComponentName;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeOnActiveSessionsChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#removeOnActiveSessionsChangedListener(android.media.session.MediaSessionManager.OnActiveSessionsChangedListener))
        ///
        /// Required features: "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_OnActiveSessionsChangedListener")))]
        pub fn removeOnActiveSessionsChangedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnActiveSessionsChangedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "removeOnActiveSessionsChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "removeOnActiveSessionsChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnActiveSessionsChangedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnSession2TokensChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#addOnSession2TokensChangedListener(android.media.session.MediaSessionManager.OnSession2TokensChangedListener))
        ///
        /// Required features: "android-media-session-MediaSessionManager_OnSession2TokensChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_OnSession2TokensChangedListener")))]
        pub fn addOnSession2TokensChangedListener_OnSession2TokensChangedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnSession2TokensChangedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "addOnSession2TokensChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "addOnSession2TokensChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addOnSession2TokensChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#addOnSession2TokensChangedListener(android.media.session.MediaSessionManager.OnSession2TokensChangedListener,%20android.os.Handler))
        ///
        /// Required features: "android-media-session-MediaSessionManager_OnSession2TokensChangedListener", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_OnSession2TokensChangedListener", feature = "android-os-Handler")))]
        pub fn addOnSession2TokensChangedListener_OnSession2TokensChangedListener_Handler<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnSession2TokensChangedListener>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "addOnSession2TokensChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;Landroid/os/Handler;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "addOnSession2TokensChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;Landroid/os/Handler;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeOnSession2TokensChangedListener](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#removeOnSession2TokensChangedListener(android.media.session.MediaSessionManager.OnSession2TokensChangedListener))
        ///
        /// Required features: "android-media-session-MediaSessionManager_OnSession2TokensChangedListener"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_OnSession2TokensChangedListener")))]
        pub fn removeOnSession2TokensChangedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_OnSession2TokensChangedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "removeOnSession2TokensChangedListener", .descriptor == "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "removeOnSession2TokensChangedListener\0", "(Landroid/media/session/MediaSessionManager$OnSession2TokensChangedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isTrustedForMediaControl](https://developer.android.com/reference/android/media/session/MediaSessionManager.html#isTrustedForMediaControl(android.media.session.MediaSessionManager.RemoteUserInfo))
        ///
        /// Required features: "android-media-session-MediaSessionManager_RemoteUserInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_RemoteUserInfo")))]
        pub fn isTrustedForMediaControl<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSessionManager_RemoteUserInfo>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/session/MediaSessionManager", java.flags == PUBLIC, .name == "isTrustedForMediaControl", .descriptor == "(Landroid/media/session/MediaSessionManager$RemoteUserInfo;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/session/MediaSessionManager\0", "isTrustedForMediaControl\0", "(Landroid/media/session/MediaSessionManager$RemoteUserInfo;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
