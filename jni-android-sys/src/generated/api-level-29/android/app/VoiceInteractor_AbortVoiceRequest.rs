// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-app-VoiceInteractor_AbortVoiceRequest"))]
__jni_bindgen! {
    /// public class [VoiceInteractor.AbortVoiceRequest](https://developer.android.com/reference/android/app/VoiceInteractor.AbortVoiceRequest.html)
    ///
    /// Required feature: android-app-VoiceInteractor_AbortVoiceRequest
    public class VoiceInteractor_AbortVoiceRequest ("android/app/VoiceInteractor$AbortVoiceRequest") extends crate::android::app::VoiceInteractor_Request {

        /// [AbortVoiceRequest](https://developer.android.com/reference/android/app/VoiceInteractor.AbortVoiceRequest.html#AbortVoiceRequest(android.app.VoiceInteractor.Prompt,%20android.os.Bundle))
        ///
        /// Required features: "android-app-VoiceInteractor_Prompt", "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-app-VoiceInteractor_Prompt", feature = "android-os-Bundle")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::app::VoiceInteractor_Prompt>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::app::VoiceInteractor_AbortVoiceRequest>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/VoiceInteractor$AbortVoiceRequest", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Landroid/app/VoiceInteractor$Prompt;Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/VoiceInteractor$AbortVoiceRequest\0", "<init>\0", "(Landroid/app/VoiceInteractor$Prompt;Landroid/os/Bundle;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onAbortResult](https://developer.android.com/reference/android/app/VoiceInteractor.AbortVoiceRequest.html#onAbortResult(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn onAbortResult<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/app/VoiceInteractor$AbortVoiceRequest", java.flags == PUBLIC, .name == "onAbortResult", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/app/VoiceInteractor$AbortVoiceRequest\0", "onAbortResult\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
