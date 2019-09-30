// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-RemoteConnection_VideoProvider_Callback"))]
__jni_bindgen! {
    /// public class [RemoteConnection.VideoProvider.Callback](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html)
    ///
    /// Required feature: android-telecom-RemoteConnection_VideoProvider_Callback
    public class RemoteConnection_VideoProvider_Callback ("android/telecom/RemoteConnection$VideoProvider$Callback") extends crate::java::lang::Object {

        /// [Callback](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#Callback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::RemoteConnection_VideoProvider_Callback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionModifyRequestReceived](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onSessionModifyRequestReceived(android.telecom.RemoteConnection.VideoProvider,%20android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider", "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider", feature = "android-telecom-VideoProfile")))]
        pub fn onSessionModifyRequestReceived<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onSessionModifyRequestReceived", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onSessionModifyRequestReceived\0", "(Landroid/telecom/RemoteConnection$VideoProvider;Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionModifyResponseReceived](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onSessionModifyResponseReceived(android.telecom.RemoteConnection.VideoProvider,%20int,%20android.telecom.VideoProfile,%20android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider", "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider", feature = "android-telecom-VideoProfile")))]
        pub fn onSessionModifyResponseReceived<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onSessionModifyResponseReceived", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;ILandroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onSessionModifyResponseReceived\0", "(Landroid/telecom/RemoteConnection$VideoProvider;ILandroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCallSessionEvent](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onCallSessionEvent(android.telecom.RemoteConnection.VideoProvider,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider")))]
        pub fn onCallSessionEvent<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onCallSessionEvent", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onCallSessionEvent\0", "(Landroid/telecom/RemoteConnection$VideoProvider;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPeerDimensionsChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onPeerDimensionsChanged(android.telecom.RemoteConnection.VideoProvider,%20int,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider")))]
        pub fn onPeerDimensionsChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onPeerDimensionsChanged", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onPeerDimensionsChanged\0", "(Landroid/telecom/RemoteConnection$VideoProvider;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCallDataUsageChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onCallDataUsageChanged(android.telecom.RemoteConnection.VideoProvider,%20long))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider")))]
        pub fn onCallDataUsageChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onCallDataUsageChanged", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onCallDataUsageChanged\0", "(Landroid/telecom/RemoteConnection$VideoProvider;J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCameraCapabilitiesChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onCameraCapabilitiesChanged(android.telecom.RemoteConnection.VideoProvider,%20android.telecom.VideoProfile.CameraCapabilities))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider", "android-telecom-VideoProfile_CameraCapabilities"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider", feature = "android-telecom-VideoProfile_CameraCapabilities")))]
        pub fn onCameraCapabilitiesChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile_CameraCapabilities>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onCameraCapabilitiesChanged", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;Landroid/telecom/VideoProfile$CameraCapabilities;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onCameraCapabilitiesChanged\0", "(Landroid/telecom/RemoteConnection$VideoProvider;Landroid/telecom/VideoProfile$CameraCapabilities;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoQualityChanged](https://developer.android.com/reference/android/telecom/RemoteConnection.VideoProvider.Callback.html#onVideoQualityChanged(android.telecom.RemoteConnection.VideoProvider,%20int))
        ///
        /// Required features: "android-telecom-RemoteConnection_VideoProvider"
        #[cfg(any(feature = "all", all(feature = "android-telecom-RemoteConnection_VideoProvider")))]
        pub fn onVideoQualityChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::RemoteConnection_VideoProvider>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/RemoteConnection$VideoProvider$Callback", java.flags == PUBLIC, .name == "onVideoQualityChanged", .descriptor == "(Landroid/telecom/RemoteConnection$VideoProvider;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/RemoteConnection$VideoProvider$Callback\0", "onVideoQualityChanged\0", "(Landroid/telecom/RemoteConnection$VideoProvider;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
