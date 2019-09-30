// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-midi-MidiSender"))]
__jni_bindgen! {
    /// public class [MidiSender](https://developer.android.com/reference/android/media/midi/MidiSender.html)
    ///
    /// Required feature: android-media-midi-MidiSender
    public class MidiSender ("android/media/midi/MidiSender") extends crate::java::lang::Object {

        /// [MidiSender](https://developer.android.com/reference/android/media/midi/MidiSender.html#MidiSender())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::midi::MidiSender>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiSender", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiSender\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/android/media/midi/MidiSender.html#connect(android.media.midi.MidiReceiver))
        ///
        /// Required features: "android-media-midi-MidiReceiver"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiReceiver")))]
        pub fn connect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::midi::MidiReceiver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiSender", java.flags == PUBLIC, .name == "connect", .descriptor == "(Landroid/media/midi/MidiReceiver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiSender\0", "connect\0", "(Landroid/media/midi/MidiReceiver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disconnect](https://developer.android.com/reference/android/media/midi/MidiSender.html#disconnect(android.media.midi.MidiReceiver))
        ///
        /// Required features: "android-media-midi-MidiReceiver"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiReceiver")))]
        pub fn disconnect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::midi::MidiReceiver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiSender", java.flags == PUBLIC, .name == "disconnect", .descriptor == "(Landroid/media/midi/MidiReceiver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiSender\0", "disconnect\0", "(Landroid/media/midi/MidiReceiver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onConnect](https://developer.android.com/reference/android/media/midi/MidiSender.html#onConnect(android.media.midi.MidiReceiver))
        ///
        /// Required features: "android-media-midi-MidiReceiver"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiReceiver")))]
        pub fn onConnect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::midi::MidiReceiver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiSender", java.flags == PUBLIC | ABSTRACT, .name == "onConnect", .descriptor == "(Landroid/media/midi/MidiReceiver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiSender\0", "onConnect\0", "(Landroid/media/midi/MidiReceiver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onDisconnect](https://developer.android.com/reference/android/media/midi/MidiSender.html#onDisconnect(android.media.midi.MidiReceiver))
        ///
        /// Required features: "android-media-midi-MidiReceiver"
        #[cfg(any(feature = "all", all(feature = "android-media-midi-MidiReceiver")))]
        pub fn onDisconnect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::media::midi::MidiReceiver>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiSender", java.flags == PUBLIC | ABSTRACT, .name == "onDisconnect", .descriptor == "(Landroid/media/midi/MidiReceiver;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiSender\0", "onDisconnect\0", "(Landroid/media/midi/MidiReceiver;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
