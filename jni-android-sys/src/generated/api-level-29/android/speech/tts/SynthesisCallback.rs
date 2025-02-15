// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-speech-tts-SynthesisCallback"))]
__jni_bindgen! {
    /// public interface [SynthesisCallback](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html)
    ///
    /// Required feature: android-speech-tts-SynthesisCallback
    public interface SynthesisCallback ("android/speech/tts/SynthesisCallback") extends crate::java::lang::Object {

        /// [getMaxBufferSize](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#getMaxBufferSize())
        pub fn getMaxBufferSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "getMaxBufferSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "getMaxBufferSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [start](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#start(int,%20int,%20int))
        pub fn start<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "start", .descriptor == "(III)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "start\0", "(III)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [audioAvailable](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#audioAvailable(byte%5B%5D,%20int,%20int))
        pub fn audioAvailable<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "audioAvailable", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "audioAvailable\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [done](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#done())
        pub fn done<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "done", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "done\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [error](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#error())
        pub fn error<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "error", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "error\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [error](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#error(int))
        pub fn error_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "error", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "error\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasStarted](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#hasStarted())
        pub fn hasStarted<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "hasStarted", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "hasStarted\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasFinished](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#hasFinished())
        pub fn hasFinished<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC | ABSTRACT, .name == "hasFinished", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "hasFinished\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [rangeStart](https://developer.android.com/reference/android/speech/tts/SynthesisCallback.html#rangeStart(int,%20int,%20int))
        pub fn rangeStart<'env>(&'env self, arg0: i32, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/speech/tts/SynthesisCallback", java.flags == PUBLIC, .name == "rangeStart", .descriptor == "(III)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/speech/tts/SynthesisCallback\0", "rangeStart\0", "(III)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
