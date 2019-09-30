// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-nfc-NfcAdapter"))]
__jni_bindgen! {
    /// public final class [NfcAdapter](https://developer.android.com/reference/android/nfc/NfcAdapter.html)
    ///
    /// Required feature: android-nfc-NfcAdapter
    public final class NfcAdapter ("android/nfc/NfcAdapter") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [NfcAdapter](https://developer.android.com/reference/android/nfc/NfcAdapter.html#NfcAdapter(android.content.Context))
        // ///
        // /// Required features: "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::nfc::NfcAdapter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/nfc/NfcAdapter", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "<init>\0", "(Landroid/content/Context;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDefaultAdapter](https://developer.android.com/reference/android/nfc/NfcAdapter.html#getDefaultAdapter(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-nfc-NfcAdapter"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-nfc-NfcAdapter")))]
        pub fn getDefaultAdapter<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::nfc::NfcAdapter>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC | STATIC, .name == "getDefaultAdapter", .descriptor == "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/nfc/NfcAdapter\0", "getDefaultAdapter\0", "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isEnabled](https://developer.android.com/reference/android/nfc/NfcAdapter.html#isEnabled())
        pub fn isEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "isEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "isEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBeamPushUris](https://developer.android.com/reference/android/nfc/NfcAdapter.html#setBeamPushUris(android.net.Uri%5B%5D,%20android.app.Activity))
        ///
        /// Required features: "android-app-Activity", "android-net-Uri"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-net-Uri")))]
        #[deprecated] pub fn setBeamPushUris<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::net::Uri, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "setBeamPushUris", .descriptor == "([Landroid/net/Uri;Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "setBeamPushUris\0", "([Landroid/net/Uri;Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBeamPushUrisCallback](https://developer.android.com/reference/android/nfc/NfcAdapter.html#setBeamPushUrisCallback(android.nfc.NfcAdapter.CreateBeamUrisCallback,%20android.app.Activity))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NfcAdapter_CreateBeamUrisCallback"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NfcAdapter_CreateBeamUrisCallback")))]
        #[deprecated] pub fn setBeamPushUrisCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter_CreateBeamUrisCallback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "setBeamPushUrisCallback", .descriptor == "(Landroid/nfc/NfcAdapter$CreateBeamUrisCallback;Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "setBeamPushUrisCallback\0", "(Landroid/nfc/NfcAdapter$CreateBeamUrisCallback;Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNdefPushMessage](https://developer.android.com/reference/android/nfc/NfcAdapter.html#setNdefPushMessage(android.nfc.NdefMessage,%20android.app.Activity,%20android.app.Activity...))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NdefMessage"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NdefMessage")))]
        #[deprecated] pub fn setNdefPushMessage<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NdefMessage>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::app::Activity, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC | VARARGS, .name == "setNdefPushMessage", .descriptor == "(Landroid/nfc/NdefMessage;Landroid/app/Activity;[Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "setNdefPushMessage\0", "(Landroid/nfc/NdefMessage;Landroid/app/Activity;[Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNdefPushMessageCallback](https://developer.android.com/reference/android/nfc/NfcAdapter.html#setNdefPushMessageCallback(android.nfc.NfcAdapter.CreateNdefMessageCallback,%20android.app.Activity,%20android.app.Activity...))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NfcAdapter_CreateNdefMessageCallback"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NfcAdapter_CreateNdefMessageCallback")))]
        #[deprecated] pub fn setNdefPushMessageCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter_CreateNdefMessageCallback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::app::Activity, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC | VARARGS, .name == "setNdefPushMessageCallback", .descriptor == "(Landroid/nfc/NfcAdapter$CreateNdefMessageCallback;Landroid/app/Activity;[Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "setNdefPushMessageCallback\0", "(Landroid/nfc/NfcAdapter$CreateNdefMessageCallback;Landroid/app/Activity;[Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setOnNdefPushCompleteCallback](https://developer.android.com/reference/android/nfc/NfcAdapter.html#setOnNdefPushCompleteCallback(android.nfc.NfcAdapter.OnNdefPushCompleteCallback,%20android.app.Activity,%20android.app.Activity...))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NfcAdapter_OnNdefPushCompleteCallback"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NfcAdapter_OnNdefPushCompleteCallback")))]
        #[deprecated] pub fn setOnNdefPushCompleteCallback<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter_OnNdefPushCompleteCallback>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::app::Activity, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC | VARARGS, .name == "setOnNdefPushCompleteCallback", .descriptor == "(Landroid/nfc/NfcAdapter$OnNdefPushCompleteCallback;Landroid/app/Activity;[Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "setOnNdefPushCompleteCallback\0", "(Landroid/nfc/NfcAdapter$OnNdefPushCompleteCallback;Landroid/app/Activity;[Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [enableForegroundDispatch](https://developer.android.com/reference/android/nfc/NfcAdapter.html#enableForegroundDispatch(android.app.Activity,%20android.app.PendingIntent,%20android.content.IntentFilter%5B%5D,%20java.lang.String%5B%5D%5B%5D))
        ///
        /// Required features: "android-app-Activity", "android-app-PendingIntent", "android-content-IntentFilter", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-app-PendingIntent", feature = "android-content-IntentFilter", feature = "java-lang-String")))]
        pub fn enableForegroundDispatch<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::PendingIntent>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::content::IntentFilter, crate::java::lang::Throwable>>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<__jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "enableForegroundDispatch", .descriptor == "(Landroid/app/Activity;Landroid/app/PendingIntent;[Landroid/content/IntentFilter;[[Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "enableForegroundDispatch\0", "(Landroid/app/Activity;Landroid/app/PendingIntent;[Landroid/content/IntentFilter;[[Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disableForegroundDispatch](https://developer.android.com/reference/android/nfc/NfcAdapter.html#disableForegroundDispatch(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        pub fn disableForegroundDispatch<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "disableForegroundDispatch", .descriptor == "(Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "disableForegroundDispatch\0", "(Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [enableReaderMode](https://developer.android.com/reference/android/nfc/NfcAdapter.html#enableReaderMode(android.app.Activity,%20android.nfc.NfcAdapter.ReaderCallback,%20int,%20android.os.Bundle))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NfcAdapter_ReaderCallback", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NfcAdapter_ReaderCallback", feature = "android-os-Bundle")))]
        pub fn enableReaderMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter_ReaderCallback>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "enableReaderMode", .descriptor == "(Landroid/app/Activity;Landroid/nfc/NfcAdapter$ReaderCallback;ILandroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "enableReaderMode\0", "(Landroid/app/Activity;Landroid/nfc/NfcAdapter$ReaderCallback;ILandroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disableReaderMode](https://developer.android.com/reference/android/nfc/NfcAdapter.html#disableReaderMode(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        pub fn disableReaderMode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "disableReaderMode", .descriptor == "(Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "disableReaderMode\0", "(Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [invokeBeam](https://developer.android.com/reference/android/nfc/NfcAdapter.html#invokeBeam(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        #[deprecated] pub fn invokeBeam<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "invokeBeam", .descriptor == "(Landroid/app/Activity;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "invokeBeam\0", "(Landroid/app/Activity;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [enableForegroundNdefPush](https://developer.android.com/reference/android/nfc/NfcAdapter.html#enableForegroundNdefPush(android.app.Activity,%20android.nfc.NdefMessage))
        ///
        /// Required features: "android-app-Activity", "android-nfc-NdefMessage"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity", feature = "android-nfc-NdefMessage")))]
        #[deprecated] pub fn enableForegroundNdefPush<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NdefMessage>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "enableForegroundNdefPush", .descriptor == "(Landroid/app/Activity;Landroid/nfc/NdefMessage;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "enableForegroundNdefPush\0", "(Landroid/app/Activity;Landroid/nfc/NdefMessage;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disableForegroundNdefPush](https://developer.android.com/reference/android/nfc/NfcAdapter.html#disableForegroundNdefPush(android.app.Activity))
        ///
        /// Required features: "android-app-Activity"
        #[cfg(any(feature = "all", all(feature = "android-app-Activity")))]
        #[deprecated] pub fn disableForegroundNdefPush<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::Activity>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "disableForegroundNdefPush", .descriptor == "(Landroid/app/Activity;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "disableForegroundNdefPush\0", "(Landroid/app/Activity;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSecureNfcSupported](https://developer.android.com/reference/android/nfc/NfcAdapter.html#isSecureNfcSupported())
        pub fn isSecureNfcSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "isSecureNfcSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "isSecureNfcSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSecureNfcEnabled](https://developer.android.com/reference/android/nfc/NfcAdapter.html#isSecureNfcEnabled())
        pub fn isSecureNfcEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "isSecureNfcEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "isSecureNfcEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isNdefPushEnabled](https://developer.android.com/reference/android/nfc/NfcAdapter.html#isNdefPushEnabled())
        #[deprecated] pub fn isNdefPushEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "isNdefPushEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "isNdefPushEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ignore](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ignore(android.nfc.Tag,%20int,%20android.nfc.NfcAdapter.OnTagRemovedListener,%20android.os.Handler))
        ///
        /// Required features: "android-nfc-NfcAdapter_OnTagRemovedListener", "android-nfc-Tag", "android-os-Handler"
        #[cfg(any(feature = "all", all(feature = "android-nfc-NfcAdapter_OnTagRemovedListener", feature = "android-nfc-Tag", feature = "android-os-Handler")))]
        pub fn ignore<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::Tag>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::nfc::NfcAdapter_OnTagRemovedListener>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Handler>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/nfc/NfcAdapter", java.flags == PUBLIC, .name == "ignore", .descriptor == "(Landroid/nfc/Tag;ILandroid/nfc/NfcAdapter$OnTagRemovedListener;Landroid/os/Handler;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/nfc/NfcAdapter\0", "ignore\0", "(Landroid/nfc/Tag;ILandroid/nfc/NfcAdapter$OnTagRemovedListener;Landroid/os/Handler;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_ADAPTER_STATE_CHANGED](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ACTION_ADAPTER_STATE_CHANGED)
        pub const ACTION_ADAPTER_STATE_CHANGED : &'static str = "android.nfc.action.ADAPTER_STATE_CHANGED";

        /// public static final [ACTION_NDEF_DISCOVERED](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ACTION_NDEF_DISCOVERED)
        pub const ACTION_NDEF_DISCOVERED : &'static str = "android.nfc.action.NDEF_DISCOVERED";

        /// public static final [ACTION_TAG_DISCOVERED](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ACTION_TAG_DISCOVERED)
        pub const ACTION_TAG_DISCOVERED : &'static str = "android.nfc.action.TAG_DISCOVERED";

        /// public static final [ACTION_TECH_DISCOVERED](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ACTION_TECH_DISCOVERED)
        pub const ACTION_TECH_DISCOVERED : &'static str = "android.nfc.action.TECH_DISCOVERED";

        /// public static final [ACTION_TRANSACTION_DETECTED](https://developer.android.com/reference/android/nfc/NfcAdapter.html#ACTION_TRANSACTION_DETECTED)
        pub const ACTION_TRANSACTION_DETECTED : &'static str = "android.nfc.action.TRANSACTION_DETECTED";

        /// public static final [EXTRA_ADAPTER_STATE](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_ADAPTER_STATE)
        pub const EXTRA_ADAPTER_STATE : &'static str = "android.nfc.extra.ADAPTER_STATE";

        /// public static final [EXTRA_AID](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_AID)
        pub const EXTRA_AID : &'static str = "android.nfc.extra.AID";

        /// public static final [EXTRA_DATA](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_DATA)
        pub const EXTRA_DATA : &'static str = "android.nfc.extra.DATA";

        /// public static final [EXTRA_ID](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_ID)
        pub const EXTRA_ID : &'static str = "android.nfc.extra.ID";

        /// public static final [EXTRA_NDEF_MESSAGES](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_NDEF_MESSAGES)
        pub const EXTRA_NDEF_MESSAGES : &'static str = "android.nfc.extra.NDEF_MESSAGES";

        /// public static final [EXTRA_READER_PRESENCE_CHECK_DELAY](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_READER_PRESENCE_CHECK_DELAY)
        pub const EXTRA_READER_PRESENCE_CHECK_DELAY : &'static str = "presence";

        /// public static final [EXTRA_SECURE_ELEMENT_NAME](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_SECURE_ELEMENT_NAME)
        pub const EXTRA_SECURE_ELEMENT_NAME : &'static str = "android.nfc.extra.SECURE_ELEMENT_NAME";

        /// public static final [EXTRA_TAG](https://developer.android.com/reference/android/nfc/NfcAdapter.html#EXTRA_TAG)
        pub const EXTRA_TAG : &'static str = "android.nfc.extra.TAG";

        /// public static final [FLAG_READER_NFC_A](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NFC_A)
        pub const FLAG_READER_NFC_A : i32 = 1;

        /// public static final [FLAG_READER_NFC_B](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NFC_B)
        pub const FLAG_READER_NFC_B : i32 = 2;

        /// public static final [FLAG_READER_NFC_BARCODE](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NFC_BARCODE)
        pub const FLAG_READER_NFC_BARCODE : i32 = 16;

        /// public static final [FLAG_READER_NFC_F](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NFC_F)
        pub const FLAG_READER_NFC_F : i32 = 4;

        /// public static final [FLAG_READER_NFC_V](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NFC_V)
        pub const FLAG_READER_NFC_V : i32 = 8;

        /// public static final [FLAG_READER_NO_PLATFORM_SOUNDS](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_NO_PLATFORM_SOUNDS)
        pub const FLAG_READER_NO_PLATFORM_SOUNDS : i32 = 256;

        /// public static final [FLAG_READER_SKIP_NDEF_CHECK](https://developer.android.com/reference/android/nfc/NfcAdapter.html#FLAG_READER_SKIP_NDEF_CHECK)
        pub const FLAG_READER_SKIP_NDEF_CHECK : i32 = 128;

        /// public static final [STATE_OFF](https://developer.android.com/reference/android/nfc/NfcAdapter.html#STATE_OFF)
        pub const STATE_OFF : i32 = 1;

        /// public static final [STATE_ON](https://developer.android.com/reference/android/nfc/NfcAdapter.html#STATE_ON)
        pub const STATE_ON : i32 = 3;

        /// public static final [STATE_TURNING_OFF](https://developer.android.com/reference/android/nfc/NfcAdapter.html#STATE_TURNING_OFF)
        pub const STATE_TURNING_OFF : i32 = 4;

        /// public static final [STATE_TURNING_ON](https://developer.android.com/reference/android/nfc/NfcAdapter.html#STATE_TURNING_ON)
        pub const STATE_TURNING_ON : i32 = 2;
    }
}
