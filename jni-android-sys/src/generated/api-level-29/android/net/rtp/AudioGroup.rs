// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-rtp-AudioGroup"))]
__jni_bindgen! {
    /// public class [AudioGroup](https://developer.android.com/reference/android/net/rtp/AudioGroup.html)
    ///
    /// Required feature: android-net-rtp-AudioGroup
    public class AudioGroup ("android/net/rtp/AudioGroup") extends crate::java::lang::Object {

        /// [AudioGroup](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#AudioGroup())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::rtp::AudioGroup>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStreams](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#getStreams())
        ///
        /// Required features: "android-net-rtp-AudioStream"
        #[cfg(any(feature = "all", all(feature = "android-net-rtp-AudioStream")))]
        pub fn getStreams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::android::net::rtp::AudioStream, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "getStreams", .descriptor == "()[Landroid/net/rtp/AudioStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "getStreams\0", "()[Landroid/net/rtp/AudioStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMode](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#getMode())
        pub fn getMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "getMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "getMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMode](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#setMode(int))
        pub fn setMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "setMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "setMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sendDtmf](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#sendDtmf(int))
        pub fn sendDtmf<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "sendDtmf", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "sendDtmf\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#clear())
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/AudioGroup", java.flags == PUBLIC, .name == "clear", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "clear\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/rtp/AudioGroup", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/AudioGroup\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [MODE_ECHO_SUPPRESSION](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#MODE_ECHO_SUPPRESSION)
        pub const MODE_ECHO_SUPPRESSION : i32 = 3;

        /// public static final [MODE_MUTED](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#MODE_MUTED)
        pub const MODE_MUTED : i32 = 1;

        /// public static final [MODE_NORMAL](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#MODE_NORMAL)
        pub const MODE_NORMAL : i32 = 2;

        /// public static final [MODE_ON_HOLD](https://developer.android.com/reference/android/net/rtp/AudioGroup.html#MODE_ON_HOLD)
        pub const MODE_ON_HOLD : i32 = 0;
    }
}
