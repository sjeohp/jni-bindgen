// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-InCallService_VideoCall_Callback"))]
__jni_bindgen! {
    /// public class [InCallService.VideoCall.Callback](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html)
    ///
    /// Required feature: android-telecom-InCallService_VideoCall_Callback
    public class InCallService_VideoCall_Callback ("android/telecom/InCallService$VideoCall$Callback") extends crate::java::lang::Object {

        /// [Callback](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#Callback())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::InCallService_VideoCall_Callback>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionModifyRequestReceived](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onSessionModifyRequestReceived(android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-VideoProfile")))]
        pub fn onSessionModifyRequestReceived<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onSessionModifyRequestReceived", .descriptor == "(Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onSessionModifyRequestReceived\0", "(Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSessionModifyResponseReceived](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onSessionModifyResponseReceived(int,%20android.telecom.VideoProfile,%20android.telecom.VideoProfile))
        ///
        /// Required features: "android-telecom-VideoProfile"
        #[cfg(any(feature = "all", all(feature = "android-telecom-VideoProfile")))]
        pub fn onSessionModifyResponseReceived<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onSessionModifyResponseReceived", .descriptor == "(ILandroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onSessionModifyResponseReceived\0", "(ILandroid/telecom/VideoProfile;Landroid/telecom/VideoProfile;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCallSessionEvent](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onCallSessionEvent(int))
        pub fn onCallSessionEvent<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onCallSessionEvent", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onCallSessionEvent\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onPeerDimensionsChanged](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onPeerDimensionsChanged(int,%20int))
        pub fn onPeerDimensionsChanged<'env>(&'env self, arg0: i32, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onPeerDimensionsChanged", .descriptor == "(II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onPeerDimensionsChanged\0", "(II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onVideoQualityChanged](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onVideoQualityChanged(int))
        pub fn onVideoQualityChanged<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onVideoQualityChanged", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onVideoQualityChanged\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCallDataUsageChanged](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onCallDataUsageChanged(long))
        pub fn onCallDataUsageChanged<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onCallDataUsageChanged", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onCallDataUsageChanged\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCameraCapabilitiesChanged](https://developer.android.com/reference/android/telecom/InCallService.VideoCall.Callback.html#onCameraCapabilitiesChanged(android.telecom.VideoProfile.CameraCapabilities))
        ///
        /// Required features: "android-telecom-VideoProfile_CameraCapabilities"
        #[cfg(any(feature = "all", all(feature = "android-telecom-VideoProfile_CameraCapabilities")))]
        pub fn onCameraCapabilitiesChanged<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::telecom::VideoProfile_CameraCapabilities>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/InCallService$VideoCall$Callback", java.flags == PUBLIC | ABSTRACT, .name == "onCameraCapabilitiesChanged", .descriptor == "(Landroid/telecom/VideoProfile$CameraCapabilities;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/InCallService$VideoCall$Callback\0", "onCameraCapabilitiesChanged\0", "(Landroid/telecom/VideoProfile$CameraCapabilities;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
