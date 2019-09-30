// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-media-MediaBrowserService"))]
__jni_bindgen! {
    /// public class [MediaBrowserService](https://developer.android.com/reference/android/service/media/MediaBrowserService.html)
    ///
    /// Required feature: android-service-media-MediaBrowserService
    public class MediaBrowserService ("android/service/media/MediaBrowserService") extends crate::android::app::Service {

        /// [MediaBrowserService](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#MediaBrowserService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::media::MediaBrowserService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCreate](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onCreate())
        pub fn onCreate<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "onCreate", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onCreate\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [dump](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#dump(java.io.FileDescriptor,%20java.io.PrintWriter,%20java.lang.String%5B%5D))
        ///
        /// Required features: "java-io-FileDescriptor", "java-io-PrintWriter", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor", feature = "java-io-PrintWriter", feature = "java-lang-String")))]
        pub fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PrintWriter>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "dump", .descriptor == "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "dump\0", "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGetRoot](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onGetRoot(java.lang.String,%20int,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-service-media-MediaBrowserService_BrowserRoot", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-service-media-MediaBrowserService_BrowserRoot", feature = "java-lang-String")))]
        pub fn onGetRoot<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::media::MediaBrowserService_BrowserRoot>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC | ABSTRACT, .name == "onGetRoot", .descriptor == "(Ljava/lang/String;ILandroid/os/Bundle;)Landroid/service/media/MediaBrowserService$BrowserRoot;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onGetRoot\0", "(Ljava/lang/String;ILandroid/os/Bundle;)Landroid/service/media/MediaBrowserService$BrowserRoot;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLoadChildren](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onLoadChildren(java.lang.String,%20android.service.media.MediaBrowserService.Result))
        ///
        /// Required features: "android-service-media-MediaBrowserService_Result", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-media-MediaBrowserService_Result", feature = "java-lang-String")))]
        pub fn onLoadChildren_String_Result<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::media::MediaBrowserService_Result>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC | ABSTRACT, .name == "onLoadChildren", .descriptor == "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onLoadChildren\0", "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLoadChildren](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onLoadChildren(java.lang.String,%20android.service.media.MediaBrowserService.Result,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-service-media-MediaBrowserService_Result", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-service-media-MediaBrowserService_Result", feature = "java-lang-String")))]
        pub fn onLoadChildren_String_Result_Bundle<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::media::MediaBrowserService_Result>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "onLoadChildren", .descriptor == "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onLoadChildren\0", "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLoadItem](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#onLoadItem(java.lang.String,%20android.service.media.MediaBrowserService.Result))
        ///
        /// Required features: "android-service-media-MediaBrowserService_Result", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-service-media-MediaBrowserService_Result", feature = "java-lang-String")))]
        pub fn onLoadItem<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::media::MediaBrowserService_Result>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "onLoadItem", .descriptor == "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "onLoadItem\0", "(Ljava/lang/String;Landroid/service/media/MediaBrowserService$Result;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSessionToken](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#setSessionToken(android.media.session.MediaSession.Token))
        ///
        /// Required features: "android-media-session-MediaSession_Token"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSession_Token")))]
        pub fn setSessionToken<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::session::MediaSession_Token>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "setSessionToken", .descriptor == "(Landroid/media/session/MediaSession$Token;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "setSessionToken\0", "(Landroid/media/session/MediaSession$Token;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSessionToken](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#getSessionToken())
        ///
        /// Required features: "android-media-session-MediaSession_Token"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSession_Token")))]
        pub fn getSessionToken<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::session::MediaSession_Token>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "getSessionToken", .descriptor == "()Landroid/media/session/MediaSession$Token;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "getSessionToken\0", "()Landroid/media/session/MediaSession$Token;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBrowserRootHints](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#getBrowserRootHints())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getBrowserRootHints<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC | FINAL, .name == "getBrowserRootHints", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "getBrowserRootHints\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCurrentBrowserInfo](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#getCurrentBrowserInfo())
        ///
        /// Required features: "android-media-session-MediaSessionManager_RemoteUserInfo"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSessionManager_RemoteUserInfo")))]
        pub fn getCurrentBrowserInfo<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::session::MediaSessionManager_RemoteUserInfo>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC | FINAL, .name == "getCurrentBrowserInfo", .descriptor == "()Landroid/media/session/MediaSessionManager$RemoteUserInfo;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "getCurrentBrowserInfo\0", "()Landroid/media/session/MediaSessionManager$RemoteUserInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyChildrenChanged](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#notifyChildrenChanged(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn notifyChildrenChanged_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "notifyChildrenChanged", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "notifyChildrenChanged\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [notifyChildrenChanged](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#notifyChildrenChanged(java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn notifyChildrenChanged_String_Bundle<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/media/MediaBrowserService", java.flags == PUBLIC, .name == "notifyChildrenChanged", .descriptor == "(Ljava/lang/String;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/media/MediaBrowserService\0", "notifyChildrenChanged\0", "(Ljava/lang/String;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/service/media/MediaBrowserService.html#SERVICE_INTERFACE)
        pub const SERVICE_INTERFACE : &'static str = "android.media.browse.MediaBrowserService";
    }
}
