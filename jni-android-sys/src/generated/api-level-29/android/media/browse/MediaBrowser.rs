// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-browse-MediaBrowser"))]
__jni_bindgen! {
    /// public final class [MediaBrowser](https://developer.android.com/reference/android/media/browse/MediaBrowser.html)
    ///
    /// Required feature: android-media-browse-MediaBrowser
    public final class MediaBrowser ("android/media/browse/MediaBrowser") extends crate::java::lang::Object {

        /// [MediaBrowser](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#MediaBrowser(android.content.Context,%20android.content.ComponentName,%20android.media.browse.MediaBrowser.ConnectionCallback,%20android.os.Bundle))
        ///
        /// Required features: "android-content-ComponentName", "android-content-Context", "android-media-browse-MediaBrowser_ConnectionCallback", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-content-Context", feature = "android-media-browse-MediaBrowser_ConnectionCallback", feature = "android-os-Bundle")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::browse::MediaBrowser_ConnectionCallback>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::browse::MediaBrowser>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/content/ComponentName;Landroid/media/browse/MediaBrowser$ConnectionCallback;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "<init>\0", "(Landroid/content/Context;Landroid/content/ComponentName;Landroid/media/browse/MediaBrowser$ConnectionCallback;Landroid/os/Bundle;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#connect())
        pub fn connect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "connect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "connect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disconnect](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#disconnect())
        pub fn disconnect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "disconnect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "disconnect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isConnected](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#isConnected())
        pub fn isConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "isConnected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "isConnected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getServiceComponent](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#getServiceComponent())
        ///
        /// Required features: "android-content-ComponentName"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName")))]
        pub fn getServiceComponent<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::ComponentName>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "getServiceComponent", .descriptor == "()Landroid/content/ComponentName;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "getServiceComponent\0", "()Landroid/content/ComponentName;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRoot](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#getRoot())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getRoot<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "getRoot", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "getRoot\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtras](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#getExtras())
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn getExtras<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Bundle>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "getExtras", .descriptor == "()Landroid/os/Bundle;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "getExtras\0", "()Landroid/os/Bundle;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSessionToken](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#getSessionToken())
        ///
        /// Required features: "android-media-session-MediaSession_Token"
        #[cfg(any(feature = "all", all(feature = "android-media-session-MediaSession_Token")))]
        pub fn getSessionToken<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::media::session::MediaSession_Token>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "getSessionToken", .descriptor == "()Landroid/media/session/MediaSession$Token;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "getSessionToken\0", "()Landroid/media/session/MediaSession$Token;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [subscribe](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#subscribe(java.lang.String,%20android.media.browse.MediaBrowser.SubscriptionCallback))
        ///
        /// Required features: "android-media-browse-MediaBrowser_SubscriptionCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-browse-MediaBrowser_SubscriptionCallback", feature = "java-lang-String")))]
        pub fn subscribe_String_SubscriptionCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::browse::MediaBrowser_SubscriptionCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "subscribe", .descriptor == "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "subscribe\0", "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [subscribe](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#subscribe(java.lang.String,%20android.os.Bundle,%20android.media.browse.MediaBrowser.SubscriptionCallback))
        ///
        /// Required features: "android-media-browse-MediaBrowser_SubscriptionCallback", "android-os-Bundle", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-browse-MediaBrowser_SubscriptionCallback", feature = "android-os-Bundle", feature = "java-lang-String")))]
        pub fn subscribe_String_Bundle_SubscriptionCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::browse::MediaBrowser_SubscriptionCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "subscribe", .descriptor == "(Ljava/lang/String;Landroid/os/Bundle;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "subscribe\0", "(Ljava/lang/String;Landroid/os/Bundle;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unsubscribe](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#unsubscribe(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn unsubscribe_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "unsubscribe", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "unsubscribe\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unsubscribe](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#unsubscribe(java.lang.String,%20android.media.browse.MediaBrowser.SubscriptionCallback))
        ///
        /// Required features: "android-media-browse-MediaBrowser_SubscriptionCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-browse-MediaBrowser_SubscriptionCallback", feature = "java-lang-String")))]
        pub fn unsubscribe_String_SubscriptionCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::browse::MediaBrowser_SubscriptionCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "unsubscribe", .descriptor == "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "unsubscribe\0", "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$SubscriptionCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getItem](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#getItem(java.lang.String,%20android.media.browse.MediaBrowser.ItemCallback))
        ///
        /// Required features: "android-media-browse-MediaBrowser_ItemCallback", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-media-browse-MediaBrowser_ItemCallback", feature = "java-lang-String")))]
        pub fn getItem<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::browse::MediaBrowser_ItemCallback>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/browse/MediaBrowser", java.flags == PUBLIC, .name == "getItem", .descriptor == "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$ItemCallback;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/browse/MediaBrowser\0", "getItem\0", "(Ljava/lang/String;Landroid/media/browse/MediaBrowser$ItemCallback;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [EXTRA_PAGE](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#EXTRA_PAGE)
        pub const EXTRA_PAGE : &'static str = "android.media.browse.extra.PAGE";

        /// public static final [EXTRA_PAGE_SIZE](https://developer.android.com/reference/android/media/browse/MediaBrowser.html#EXTRA_PAGE_SIZE)
        pub const EXTRA_PAGE_SIZE : &'static str = "android.media.browse.extra.PAGE_SIZE";
    }
}
