// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-speech-RecognizerIntent"))]
__jni_bindgen! {
    /// public class [RecognizerIntent](https://developer.android.com/reference/android/speech/RecognizerIntent.html)
    ///
    /// Required feature: android-speech-RecognizerIntent
    public class RecognizerIntent ("android/speech/RecognizerIntent") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [RecognizerIntent](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RecognizerIntent())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::speech::RecognizerIntent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/speech/RecognizerIntent", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/RecognizerIntent\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getVoiceDetailsIntent](https://developer.android.com/reference/android/speech/RecognizerIntent.html#getVoiceDetailsIntent(android.content.Context))
        ///
        /// Required features: "android-content-Context", "android-content-Intent"
        #[cfg(any(feature = "all", all(feature = "android-content-Context", feature = "android-content-Intent")))]
        pub fn getVoiceDetailsIntent<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::content::Intent>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/RecognizerIntent", java.flags == PUBLIC | STATIC | FINAL, .name == "getVoiceDetailsIntent", .descriptor == "(Landroid/content/Context;)Landroid/content/Intent;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/speech/RecognizerIntent\0", "getVoiceDetailsIntent\0", "(Landroid/content/Context;)Landroid/content/Intent;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [ACTION_GET_LANGUAGE_DETAILS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#ACTION_GET_LANGUAGE_DETAILS)
        pub const ACTION_GET_LANGUAGE_DETAILS : &'static str = "android.speech.action.GET_LANGUAGE_DETAILS";

        /// public static final [ACTION_RECOGNIZE_SPEECH](https://developer.android.com/reference/android/speech/RecognizerIntent.html#ACTION_RECOGNIZE_SPEECH)
        pub const ACTION_RECOGNIZE_SPEECH : &'static str = "android.speech.action.RECOGNIZE_SPEECH";

        /// public static final [ACTION_VOICE_SEARCH_HANDS_FREE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#ACTION_VOICE_SEARCH_HANDS_FREE)
        pub const ACTION_VOICE_SEARCH_HANDS_FREE : &'static str = "android.speech.action.VOICE_SEARCH_HANDS_FREE";

        /// public static final [ACTION_WEB_SEARCH](https://developer.android.com/reference/android/speech/RecognizerIntent.html#ACTION_WEB_SEARCH)
        pub const ACTION_WEB_SEARCH : &'static str = "android.speech.action.WEB_SEARCH";

        /// public static final [DETAILS_META_DATA](https://developer.android.com/reference/android/speech/RecognizerIntent.html#DETAILS_META_DATA)
        pub const DETAILS_META_DATA : &'static str = "android.speech.DETAILS";

        /// public static final [EXTRA_CALLING_PACKAGE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_CALLING_PACKAGE)
        pub const EXTRA_CALLING_PACKAGE : &'static str = "calling_package";

        /// public static final [EXTRA_CONFIDENCE_SCORES](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_CONFIDENCE_SCORES)
        pub const EXTRA_CONFIDENCE_SCORES : &'static str = "android.speech.extra.CONFIDENCE_SCORES";

        /// public static final [EXTRA_LANGUAGE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_LANGUAGE)
        pub const EXTRA_LANGUAGE : &'static str = "android.speech.extra.LANGUAGE";

        /// public static final [EXTRA_LANGUAGE_MODEL](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_LANGUAGE_MODEL)
        pub const EXTRA_LANGUAGE_MODEL : &'static str = "android.speech.extra.LANGUAGE_MODEL";

        /// public static final [EXTRA_LANGUAGE_PREFERENCE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_LANGUAGE_PREFERENCE)
        pub const EXTRA_LANGUAGE_PREFERENCE : &'static str = "android.speech.extra.LANGUAGE_PREFERENCE";

        /// public static final [EXTRA_MAX_RESULTS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_MAX_RESULTS)
        pub const EXTRA_MAX_RESULTS : &'static str = "android.speech.extra.MAX_RESULTS";

        /// public static final [EXTRA_ONLY_RETURN_LANGUAGE_PREFERENCE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_ONLY_RETURN_LANGUAGE_PREFERENCE)
        pub const EXTRA_ONLY_RETURN_LANGUAGE_PREFERENCE : &'static str = "android.speech.extra.ONLY_RETURN_LANGUAGE_PREFERENCE";

        /// public static final [EXTRA_ORIGIN](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_ORIGIN)
        pub const EXTRA_ORIGIN : &'static str = "android.speech.extra.ORIGIN";

        /// public static final [EXTRA_PARTIAL_RESULTS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_PARTIAL_RESULTS)
        pub const EXTRA_PARTIAL_RESULTS : &'static str = "android.speech.extra.PARTIAL_RESULTS";

        /// public static final [EXTRA_PREFER_OFFLINE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_PREFER_OFFLINE)
        pub const EXTRA_PREFER_OFFLINE : &'static str = "android.speech.extra.PREFER_OFFLINE";

        /// public static final [EXTRA_PROMPT](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_PROMPT)
        pub const EXTRA_PROMPT : &'static str = "android.speech.extra.PROMPT";

        /// public static final [EXTRA_RESULTS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_RESULTS)
        pub const EXTRA_RESULTS : &'static str = "android.speech.extra.RESULTS";

        /// public static final [EXTRA_RESULTS_PENDINGINTENT](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_RESULTS_PENDINGINTENT)
        pub const EXTRA_RESULTS_PENDINGINTENT : &'static str = "android.speech.extra.RESULTS_PENDINGINTENT";

        /// public static final [EXTRA_RESULTS_PENDINGINTENT_BUNDLE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_RESULTS_PENDINGINTENT_BUNDLE)
        pub const EXTRA_RESULTS_PENDINGINTENT_BUNDLE : &'static str = "android.speech.extra.RESULTS_PENDINGINTENT_BUNDLE";

        /// public static final [EXTRA_SECURE](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_SECURE)
        pub const EXTRA_SECURE : &'static str = "android.speech.extras.EXTRA_SECURE";

        /// public static final [EXTRA_SPEECH_INPUT_COMPLETE_SILENCE_LENGTH_MILLIS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_SPEECH_INPUT_COMPLETE_SILENCE_LENGTH_MILLIS)
        pub const EXTRA_SPEECH_INPUT_COMPLETE_SILENCE_LENGTH_MILLIS : &'static str = "android.speech.extras.SPEECH_INPUT_COMPLETE_SILENCE_LENGTH_MILLIS";

        /// public static final [EXTRA_SPEECH_INPUT_MINIMUM_LENGTH_MILLIS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_SPEECH_INPUT_MINIMUM_LENGTH_MILLIS)
        pub const EXTRA_SPEECH_INPUT_MINIMUM_LENGTH_MILLIS : &'static str = "android.speech.extras.SPEECH_INPUT_MINIMUM_LENGTH_MILLIS";

        /// public static final [EXTRA_SPEECH_INPUT_POSSIBLY_COMPLETE_SILENCE_LENGTH_MILLIS](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_SPEECH_INPUT_POSSIBLY_COMPLETE_SILENCE_LENGTH_MILLIS)
        pub const EXTRA_SPEECH_INPUT_POSSIBLY_COMPLETE_SILENCE_LENGTH_MILLIS : &'static str = "android.speech.extras.SPEECH_INPUT_POSSIBLY_COMPLETE_SILENCE_LENGTH_MILLIS";

        /// public static final [EXTRA_SUPPORTED_LANGUAGES](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_SUPPORTED_LANGUAGES)
        pub const EXTRA_SUPPORTED_LANGUAGES : &'static str = "android.speech.extra.SUPPORTED_LANGUAGES";

        /// public static final [EXTRA_WEB_SEARCH_ONLY](https://developer.android.com/reference/android/speech/RecognizerIntent.html#EXTRA_WEB_SEARCH_ONLY)
        pub const EXTRA_WEB_SEARCH_ONLY : &'static str = "android.speech.extra.WEB_SEARCH_ONLY";

        /// public static final [LANGUAGE_MODEL_FREE_FORM](https://developer.android.com/reference/android/speech/RecognizerIntent.html#LANGUAGE_MODEL_FREE_FORM)
        pub const LANGUAGE_MODEL_FREE_FORM : &'static str = "free_form";

        /// public static final [LANGUAGE_MODEL_WEB_SEARCH](https://developer.android.com/reference/android/speech/RecognizerIntent.html#LANGUAGE_MODEL_WEB_SEARCH)
        pub const LANGUAGE_MODEL_WEB_SEARCH : &'static str = "web_search";

        /// public static final [RESULT_AUDIO_ERROR](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RESULT_AUDIO_ERROR)
        pub const RESULT_AUDIO_ERROR : i32 = 5;

        /// public static final [RESULT_CLIENT_ERROR](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RESULT_CLIENT_ERROR)
        pub const RESULT_CLIENT_ERROR : i32 = 2;

        /// public static final [RESULT_NETWORK_ERROR](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RESULT_NETWORK_ERROR)
        pub const RESULT_NETWORK_ERROR : i32 = 4;

        /// public static final [RESULT_NO_MATCH](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RESULT_NO_MATCH)
        pub const RESULT_NO_MATCH : i32 = 1;

        /// public static final [RESULT_SERVER_ERROR](https://developer.android.com/reference/android/speech/RecognizerIntent.html#RESULT_SERVER_ERROR)
        pub const RESULT_SERVER_ERROR : i32 = 3;
    }
}
