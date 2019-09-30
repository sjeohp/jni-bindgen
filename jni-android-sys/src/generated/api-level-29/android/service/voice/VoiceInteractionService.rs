// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-service-voice-VoiceInteractionService"))]
__jni_bindgen! {
    /// public class [VoiceInteractionService](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html)
    ///
    /// Required feature: android-service-voice-VoiceInteractionService
    public class VoiceInteractionService ("android/service/voice/VoiceInteractionService") extends crate::android::app::Service {

        /// [VoiceInteractionService](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#VoiceInteractionService())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::service::voice::VoiceInteractionService>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onLaunchVoiceAssistFromKeyguard](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#onLaunchVoiceAssistFromKeyguard())
        pub fn onLaunchVoiceAssistFromKeyguard<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "onLaunchVoiceAssistFromKeyguard", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "onLaunchVoiceAssistFromKeyguard\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isActiveService](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#isActiveService(android.content.Context,%20android.content.ComponentName))
        ///
        /// Required features: "android-content-ComponentName", "android-content-Context"
        #[cfg(any(feature = "all", all(feature = "android-content-ComponentName", feature = "android-content-Context")))]
        pub fn isActiveService<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Context>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::ComponentName>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC | STATIC, .name == "isActiveService", .descriptor == "(Landroid/content/Context;Landroid/content/ComponentName;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/service/voice/VoiceInteractionService\0", "isActiveService\0", "(Landroid/content/Context;Landroid/content/ComponentName;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDisabledShowContext](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#setDisabledShowContext(int))
        pub fn setDisabledShowContext<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "setDisabledShowContext", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "setDisabledShowContext\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDisabledShowContext](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#getDisabledShowContext())
        pub fn getDisabledShowContext<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "getDisabledShowContext", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "getDisabledShowContext\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [showSession](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#showSession(android.os.Bundle,%20int))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn showSession<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "showSession", .descriptor == "(Landroid/os/Bundle;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "showSession\0", "(Landroid/os/Bundle;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onGetSupportedVoiceActions](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#onGetSupportedVoiceActions(java.util.Set))
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn onGetSupportedVoiceActions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "onGetSupportedVoiceActions", .descriptor == "(Ljava/util/Set;)Ljava/util/Set;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "onGetSupportedVoiceActions\0", "(Ljava/util/Set;)Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onBind](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#onBind(android.content.Intent))
        ///
        /// Required features: "android-content-Intent", "android-os-IBinder"
        #[cfg(any(feature = "all", all(feature = "android-content-Intent", feature = "android-os-IBinder")))]
        pub fn onBind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::content::Intent>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::IBinder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "onBind", .descriptor == "(Landroid/content/Intent;)Landroid/os/IBinder;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "onBind\0", "(Landroid/content/Intent;)Landroid/os/IBinder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onReady](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#onReady())
        pub fn onReady<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "onReady", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "onReady\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onShutdown](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#onShutdown())
        pub fn onShutdown<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC, .name == "onShutdown", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "onShutdown\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createAlwaysOnHotwordDetector](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#createAlwaysOnHotwordDetector(java.lang.String,%20java.util.Locale,%20android.service.voice.AlwaysOnHotwordDetector.Callback))
        ///
        /// Required features: "android-service-voice-AlwaysOnHotwordDetector", "android-service-voice-AlwaysOnHotwordDetector_Callback", "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "android-service-voice-AlwaysOnHotwordDetector", feature = "android-service-voice-AlwaysOnHotwordDetector_Callback", feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn createAlwaysOnHotwordDetector<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::service::voice::AlwaysOnHotwordDetector_Callback>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::service::voice::AlwaysOnHotwordDetector>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC | FINAL, .name == "createAlwaysOnHotwordDetector", .descriptor == "(Ljava/lang/String;Ljava/util/Locale;Landroid/service/voice/AlwaysOnHotwordDetector$Callback;)Landroid/service/voice/AlwaysOnHotwordDetector;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "createAlwaysOnHotwordDetector\0", "(Ljava/lang/String;Ljava/util/Locale;Landroid/service/voice/AlwaysOnHotwordDetector$Callback;)Landroid/service/voice/AlwaysOnHotwordDetector;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setUiHints](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#setUiHints(android.os.Bundle))
        ///
        /// Required features: "android-os-Bundle"
        #[cfg(any(feature = "all", all(feature = "android-os-Bundle")))]
        pub fn setUiHints<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::os::Bundle>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/service/voice/VoiceInteractionService", java.flags == PUBLIC | FINAL, .name == "setUiHints", .descriptor == "(Landroid/os/Bundle;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "setUiHints\0", "(Landroid/os/Bundle;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [dump](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#dump(java.io.FileDescriptor,%20java.io.PrintWriter,%20java.lang.String%5B%5D))
        // ///
        // /// Required features: "java-io-FileDescriptor", "java-io-PrintWriter", "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor", feature = "java-io-PrintWriter", feature = "java-lang-String")))]
        // fn dump<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PrintWriter>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/service/voice/VoiceInteractionService", java.flags == PROTECTED, .name == "dump", .descriptor == "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/service/voice/VoiceInteractionService\0", "dump\0", "(Ljava/io/FileDescriptor;Ljava/io/PrintWriter;[Ljava/lang/String;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [SERVICE_INTERFACE](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#SERVICE_INTERFACE)
        pub const SERVICE_INTERFACE : &'static str = "android.service.voice.VoiceInteractionService";

        /// public static final [SERVICE_META_DATA](https://developer.android.com/reference/android/service/voice/VoiceInteractionService.html#SERVICE_META_DATA)
        pub const SERVICE_META_DATA : &'static str = "android.voice_interaction";
    }
}
