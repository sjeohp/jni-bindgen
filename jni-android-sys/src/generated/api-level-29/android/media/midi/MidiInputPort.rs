// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-media-midi-MidiInputPort"))]
__jni_bindgen! {
    /// public final class [MidiInputPort](https://developer.android.com/reference/android/media/midi/MidiInputPort.html)
    ///
    /// Required feature: android-media-midi-MidiInputPort
    public final class MidiInputPort ("android/media/midi/MidiInputPort") extends crate::android::media::midi::MidiReceiver, implements crate::java::io::Closeable {

        // // Not emitting: Non-public method
        // /// [MidiInputPort](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#MidiInputPort(java.io.FileDescriptor,%20int))
        // ///
        // /// Required features: "java-io-FileDescriptor"
        // #[cfg(any(feature = "all", all(feature = "java-io-FileDescriptor")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::FileDescriptor>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::media::midi::MidiInputPort>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/midi/MidiInputPort", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/io/FileDescriptor;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "<init>\0", "(Ljava/io/FileDescriptor;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getPortNumber](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#getPortNumber())
        pub fn getPortNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiInputPort", java.flags == PUBLIC, .name == "getPortNumber", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "getPortNumber\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onSend](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#onSend(byte%5B%5D,%20int,%20int,%20long))
        pub fn onSend<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32, arg3: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiInputPort", java.flags == PUBLIC, .name == "onSend", .descriptor == "([BIIJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "onSend\0", "([BIIJ)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onFlush](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#onFlush())
        pub fn onFlush<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiInputPort", java.flags == PUBLIC, .name == "onFlush", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "onFlush\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/media/midi/MidiInputPort", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/media/midi/MidiInputPort.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/media/midi/MidiInputPort", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/media/midi/MidiInputPort\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
