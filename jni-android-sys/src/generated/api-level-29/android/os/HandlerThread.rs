// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-os-HandlerThread"))]
__jni_bindgen! {
    /// public class [HandlerThread](https://developer.android.com/reference/android/os/HandlerThread.html)
    ///
    /// Required feature: android-os-HandlerThread
    public class HandlerThread ("android/os/HandlerThread") extends crate::java::lang::Thread {

        /// [HandlerThread](https://developer.android.com/reference/android/os/HandlerThread.html#HandlerThread(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::HandlerThread>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [HandlerThread](https://developer.android.com/reference/android/os/HandlerThread.html#HandlerThread(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::os::HandlerThread>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "<init>\0", "(Ljava/lang/String;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [onLooperPrepared](https://developer.android.com/reference/android/os/HandlerThread.html#onLooperPrepared())
        // fn onLooperPrepared<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/os/HandlerThread", java.flags == PROTECTED, .name == "onLooperPrepared", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "onLooperPrepared\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [run](https://developer.android.com/reference/android/os/HandlerThread.html#run())
        pub fn run<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "run", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "run\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLooper](https://developer.android.com/reference/android/os/HandlerThread.html#getLooper())
        ///
        /// Required features: "android-os-Looper"
        #[cfg(any(feature = "all", all(feature = "android-os-Looper")))]
        pub fn getLooper<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::android::os::Looper>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "getLooper", .descriptor == "()Landroid/os/Looper;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "getLooper\0", "()Landroid/os/Looper;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [quit](https://developer.android.com/reference/android/os/HandlerThread.html#quit())
        pub fn quit<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "quit", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "quit\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [quitSafely](https://developer.android.com/reference/android/os/HandlerThread.html#quitSafely())
        pub fn quitSafely<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "quitSafely", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "quitSafely\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getThreadId](https://developer.android.com/reference/android/os/HandlerThread.html#getThreadId())
        pub fn getThreadId<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/os/HandlerThread", java.flags == PUBLIC, .name == "getThreadId", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/os/HandlerThread\0", "getThreadId\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
