// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-RemoteConnection_Callback"))]
__jni_bindgen! {
    /// public class [RemoteConnection.Callback](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html)
    ///
    /// Required feature: android-telecom-RemoteConnection_Callback
    public class RemoteConnection_Callback ("android/telecom/RemoteConnection$Callback") extends crate::java::lang::Object {

        /// [Callback](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#Callback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::RemoteConnection_Callback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStateChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onStateChanged(android.telecom.RemoteConnection,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onStateChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onStateChanged", .descriptor == "(Landroid/telecom/RemoteConnection;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onStateChanged\0", "(Landroid/telecom/RemoteConnection;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnected](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onDisconnected(android.telecom.RemoteConnection,%20android.telecom.DisconnectCause))
        ///
        /// Required features: "android-telecom-DisconnectCause", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-DisconnectCause", feature = "android-telecom-RemoteConnection")))]
        pub fn onDisconnected<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::DisconnectCause>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onDisconnected", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/telecom/DisconnectCause;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onDisconnected\0", "(Landroid/telecom/RemoteConnection;Landroid/telecom/DisconnectCause;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRingbackRequested](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onRingbackRequested(android.telecom.RemoteConnection,%20boolean))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onRingbackRequested<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onRingbackRequested", .descriptor == "(Landroid/telecom/RemoteConnection;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onRingbackRequested\0", "(Landroid/telecom/RemoteConnection;Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionCapabilitiesChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onConnectionCapabilitiesChanged(android.telecom.RemoteConnection,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onConnectionCapabilitiesChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onConnectionCapabilitiesChanged", .descriptor == "(Landroid/telecom/RemoteConnection;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onConnectionCapabilitiesChanged\0", "(Landroid/telecom/RemoteConnection;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionPropertiesChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onConnectionPropertiesChanged(android.telecom.RemoteConnection,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onConnectionPropertiesChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onConnectionPropertiesChanged", .descriptor == "(Landroid/telecom/RemoteConnection;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onConnectionPropertiesChanged\0", "(Landroid/telecom/RemoteConnection;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPostDialWait](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onPostDialWait(android.telecom.RemoteConnection,%20java.lang.String))
        ///
        /// Required features: "android-telecom-RemoteConnection", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection", feature = "java-lang-String")))]
        pub fn onPostDialWait<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onPostDialWait", .descriptor == "(Landroid/telecom/RemoteConnection;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onPostDialWait\0", "(Landroid/telecom/RemoteConnection;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPostDialChar](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onPostDialChar(android.telecom.RemoteConnection,%20char))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onPostDialChar<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onPostDialChar", .descriptor == "(Landroid/telecom/RemoteConnection;C)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onPostDialChar\0", "(Landroid/telecom/RemoteConnection;C)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVoipAudioChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onVoipAudioChanged(android.telecom.RemoteConnection,%20boolean))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onVoipAudioChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onVoipAudioChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onVoipAudioChanged\0", "(Landroid/telecom/RemoteConnection;Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onStatusHintsChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onStatusHintsChanged(android.telecom.RemoteConnection,%20android.telecom.StatusHints))
        ///
        /// Required features: "android-telecom-RemoteConnection", "android-telecom-StatusHints"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection", feature = "android-telecom-StatusHints")))]
        pub fn onStatusHintsChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::StatusHints>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onStatusHintsChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/telecom/StatusHints;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onStatusHintsChanged\0", "(Landroid/telecom/RemoteConnection;Landroid/telecom/StatusHints;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAddressChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onAddressChanged(android.telecom.RemoteConnection,%20android.net.Uri,%20int))
        ///
        /// Required features: "android-net-Uri", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-net-Uri", feature = "android-telecom-RemoteConnection")))]
        pub fn onAddressChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::Uri>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onAddressChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/net/Uri;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onAddressChanged\0", "(Landroid/telecom/RemoteConnection;Landroid/net/Uri;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCallerDisplayNameChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onCallerDisplayNameChanged(android.telecom.RemoteConnection,%20java.lang.String,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection", feature = "java-lang-String")))]
        pub fn onCallerDisplayNameChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onCallerDisplayNameChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onCallerDisplayNameChanged\0", "(Landroid/telecom/RemoteConnection;Ljava/lang/String;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoStateChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onVideoStateChanged(android.telecom.RemoteConnection,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onVideoStateChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onVideoStateChanged", .descriptor == "(Landroid/telecom/RemoteConnection;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onVideoStateChanged\0", "(Landroid/telecom/RemoteConnection;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDestroyed](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onDestroyed(android.telecom.RemoteConnection))
        ///
        /// Required features: "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection")))]
        pub fn onDestroyed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onDestroyed", .descriptor == "(Landroid/telecom/RemoteConnection;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onDestroyed\0", "(Landroid/telecom/RemoteConnection;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConferenceableConnectionsChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onConferenceableConnectionsChanged(android.telecom.RemoteConnection,%20java.util.List))
        ///
        /// Required features: "android-telecom-RemoteConnection", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection", feature = "java-util-List")))]
        pub fn onConferenceableConnectionsChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onConferenceableConnectionsChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onConferenceableConnectionsChanged\0", "(Landroid/telecom/RemoteConnection;Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoProviderChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onVideoProviderChanged(android.telecom.RemoteConnection,%20android.telecom.RemoteConnection.VideoProvider))
        ///
        /// Required features: "android-telecom-RemoteConnection", "android-telecom-RemoteConnection_VideoProvider"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection", feature = "android-telecom-RemoteConnection_VideoProvider")))]
        pub fn onVideoProviderChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onVideoProviderChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConnection$VideoProvider;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onVideoProviderChanged\0", "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConnection$VideoProvider;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConferenceChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onConferenceChanged(android.telecom.RemoteConnection,%20android.telecom.RemoteConference))
        ///
        /// Required features: "android-telecom-RemoteConference", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConference", feature = "android-telecom-RemoteConnection")))]
        pub fn onConferenceChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConference>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onConferenceChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConference;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onConferenceChanged\0", "(Landroid/telecom/RemoteConnection;Landroid/telecom/RemoteConference;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onExtrasChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onExtrasChanged(android.telecom.RemoteConnection,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-telecom-RemoteConnection"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-telecom-RemoteConnection")))]
        pub fn onExtrasChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onExtrasChanged", .descriptor == "(Landroid/telecom/RemoteConnection;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onExtrasChanged\0", "(Landroid/telecom/RemoteConnection;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnectionEvent](https://developer.android.com/reference/android/telecom/RemoteConnection.Callback.html#onConnectionEvent(android.telecom.RemoteConnection,%20java.lang.String,%20android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle", "android-telecom-RemoteConnection", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle", feature = "android-telecom-RemoteConnection", feature = "java-lang-String")))]
        pub fn onConnectionEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$Callback", java.flags == PUBLIC, .name == "onConnectionEvent", .descriptor == "(Landroid/telecom/RemoteConnection;Ljava/lang/String;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$Callback\0", "onConnectionEvent\0", "(Landroid/telecom/RemoteConnection;Ljava/lang/String;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
