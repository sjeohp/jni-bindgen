// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-speech-SpeechRecognizer"))]
__jni_bindgen! {
    /// public class [SpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer.html)
    ///
    /// Required feature: android-speech-SpeechRecognizer
    public class SpeechRecognizer ("android/speech/SpeechRecognizer") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#SpeechRecognizer(android.content.Context,%20android.content.ComponentName))
        // ///
        // /// Required features: "android-content-ComponentName", "android-content-Context"
        // #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-content-Context")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::speech::SpeechRecognizer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/speech/SpeechRecognizer", java.flags == (empty), .name == "<init>", .descriptor == "(Landroid/content/Context;Landroid/content/ComponentName;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "<init>\0", "(Landroid/content/Context;Landroid/content/ComponentName;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isRecognitionAvailable](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#isRecognitionAvailable(android.content.Context))
        ///
        /// Required features: "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-Context")))]
        pub fn isRecognitionAvailable<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC | STATIC, .name == "isRecognitionAvailable", .descriptor == "(Landroid/content/Context;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/speech/SpeechRecognizer\0", "isRecognitionAvailable\0", "(Landroid/content/Context;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#createSpeechRecognizer(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-speech-SpeechRecognizer"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-speech-SpeechRecognizer")))]
        pub fn createSpeechRecognizer_Context<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::speech::SpeechRecognizer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC | STATIC, .name == "createSpeechRecognizer", .descriptor == "(Landroid/content/Context;)Landroid/speech/SpeechRecognizer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/speech/SpeechRecognizer\0", "createSpeechRecognizer\0", "(Landroid/content/Context;)Landroid/speech/SpeechRecognizer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSpeechRecognizer](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#createSpeechRecognizer(android.content.Context,%20android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName", "android-content-Context", "android-speech-SpeechRecognizer"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-content-Context", feature = "android-speech-SpeechRecognizer")))]
        pub fn createSpeechRecognizer_Context_ComponentName<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::speech::SpeechRecognizer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC | STATIC, .name == "createSpeechRecognizer", .descriptor == "(Landroid/content/Context;Landroid/content/ComponentName;)Landroid/speech/SpeechRecognizer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/speech/SpeechRecognizer\0", "createSpeechRecognizer\0", "(Landroid/content/Context;Landroid/content/ComponentName;)Landroid/speech/SpeechRecognizer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRecognitionListener](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#setRecognitionListener(android.speech.RecognitionListener))
        ///
        /// Required features: "android-speech-RecognitionListener"
        #[cfg(any(feature = "all", all(feature = "android-speech-RecognitionListener")))]
        pub fn setRecognitionListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::speech::RecognitionListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC, .name == "setRecognitionListener", .descriptor == "(Landroid/speech/RecognitionListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "setRecognitionListener\0", "(Landroid/speech/RecognitionListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startListening](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#startListening(android.content.Intent))
        ///
        /// Required features: "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent")))]
        pub fn startListening<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC, .name == "startListening", .descriptor == "(Landroid/content/Intent;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "startListening\0", "(Landroid/content/Intent;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [stopListening](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#stopListening())
        pub fn stopListening<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC, .name == "stopListening", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "stopListening\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancel](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#cancel())
        pub fn cancel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC, .name == "cancel", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "cancel\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [destroy](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#destroy())
        pub fn destroy<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/SpeechRecognizer", java.flags == PUBLIC, .name == "destroy", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/SpeechRecognizer\0", "destroy\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [CONFIDENCE_SCORES](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#CONFIDENCE_SCORES)
        pub const CONFIDENCE_SCORES : &'static str = "confidence_scores";

        /// public static final [ERROR_AUDIO](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_AUDIO)
        pub const ERROR_AUDIO : i32 = 3;

        /// public static final [ERROR_CLIENT](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_CLIENT)
        pub const ERROR_CLIENT : i32 = 5;

        /// public static final [ERROR_INSUFFICIENT_PERMISSIONS](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_INSUFFICIENT_PERMISSIONS)
        pub const ERROR_INSUFFICIENT_PERMISSIONS : i32 = 9;

        /// public static final [ERROR_NETWORK](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_NETWORK)
        pub const ERROR_NETWORK : i32 = 2;

        /// public static final [ERROR_NETWORK_TIMEOUT](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_NETWORK_TIMEOUT)
        pub const ERROR_NETWORK_TIMEOUT : i32 = 1;

        /// public static final [ERROR_NO_MATCH](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_NO_MATCH)
        pub const ERROR_NO_MATCH : i32 = 7;

        /// public static final [ERROR_RECOGNIZER_BUSY](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_RECOGNIZER_BUSY)
        pub const ERROR_RECOGNIZER_BUSY : i32 = 8;

        /// public static final [ERROR_SERVER](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_SERVER)
        pub const ERROR_SERVER : i32 = 4;

        /// public static final [ERROR_SPEECH_TIMEOUT](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#ERROR_SPEECH_TIMEOUT)
        pub const ERROR_SPEECH_TIMEOUT : i32 = 6;

        /// public static final [RESULTS_RECOGNITION](https://developer.android.com/reference/android/speech/SpeechRecognizer.html#RESULTS_RECOGNITION)
        pub const RESULTS_RECOGNITION : &'static str = "results_recognition";
    }
}
