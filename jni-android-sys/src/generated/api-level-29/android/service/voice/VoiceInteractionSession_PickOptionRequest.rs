// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-voice-VoiceInteractionSession_PickOptionRequest"))]
__jni_bindgen! {
    /// public final class [VoiceInteractionSession.PickOptionRequest](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html)
    ///
    /// Required feature: android-service-voice-VoiceInteractionSession_PickOptionRequest
    public final class VoiceInteractionSession_PickOptionRequest ("android/service/voice/VoiceInteractionSession$PickOptionRequest") extends crate::android::service::voice::VoiceInteractionSession_Request {

        // // Not emitting: Non-public method
        // /// [PickOptionRequest](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#PickOptionRequest())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::voice::VoiceInteractionSession_PickOptionRequest>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getVoicePrompt](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#getVoicePrompt())
        ///
        /// Required features: "android-app-VoiceInteractor_Prompt"
        #[cfg(any(feature = "all", all(feature = "android-app-VoiceInteractor_Prompt")))]
        pub fn getVoicePrompt<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::app::VoiceInteractor_Prompt>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == PUBLIC, .name == "getVoicePrompt", .descriptor == "()Landroid/app/VoiceInteractor$Prompt;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "getVoicePrompt\0", "()Landroid/app/VoiceInteractor$Prompt;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrompt](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#getPrompt())
        ///
        /// Required features: "java-lang-CharSequence"
        #[cfg(any(feature = "all", all(feature = "java-lang-CharSequence")))]
        #[deprecated] pub fn getPrompt<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::CharSequence>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == PUBLIC, .name == "getPrompt", .descriptor == "()Ljava/lang/CharSequence;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "getPrompt\0", "()Ljava/lang/CharSequence;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getOptions](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#getOptions())
        ///
        /// Required features: "android-app-VoiceInteractor_PickOptionRequest_Option"
        #[cfg(any(feature = "all", all(feature = "android-app-VoiceInteractor_PickOptionRequest_Option")))]
        pub fn getOptions<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::app::VoiceInteractor_PickOptionRequest_Option, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == PUBLIC, .name == "getOptions", .descriptor == "()[Landroid/app/VoiceInteractor$PickOptionRequest$Option;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "getOptions\0", "()[Landroid/app/VoiceInteractor$PickOptionRequest$Option;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendIntermediatePickOptionResult](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#sendIntermediatePickOptionResult(android.app.VoiceInteractor.PickOptionRequest.Option%5B%5D,%20android.os.Bundle))
        ///
        /// Required features: "android-app-VoiceInteractor_PickOptionRequest_Option", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-VoiceInteractor_PickOptionRequest_Option", feature = "android-os-Bundle")))]
        pub fn sendIntermediatePickOptionResult<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::app::VoiceInteractor_PickOptionRequest_Option, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == PUBLIC, .name == "sendIntermediatePickOptionResult", .descriptor == "([Landroid/app/VoiceInteractor$PickOptionRequest$Option;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "sendIntermediatePickOptionResult\0", "([Landroid/app/VoiceInteractor$PickOptionRequest$Option;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendPickOptionResult](https://developer.android.com/reference/android/service/voice/VoiceInteractionSession.PickOptionRequest.html#sendPickOptionResult(android.app.VoiceInteractor.PickOptionRequest.Option%5B%5D,%20android.os.Bundle))
        ///
        /// Required features: "android-app-VoiceInteractor_PickOptionRequest_Option", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-VoiceInteractor_PickOptionRequest_Option", feature = "android-os-Bundle")))]
        pub fn sendPickOptionResult<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::android::app::VoiceInteractor_PickOptionRequest_Option, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionSession$PickOptionRequest", java.flags == PUBLIC, .name == "sendPickOptionResult", .descriptor == "([Landroid/app/VoiceInteractor$PickOptionRequest$Option;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionSession$PickOptionRequest\0", "sendPickOptionResult\0", "([Landroid/app/VoiceInteractor$PickOptionRequest$Option;Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
