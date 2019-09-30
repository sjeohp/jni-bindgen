// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-telecom-Call_RttCall"))]
__jni_bindgen! {
    /// public final class [Call.RttCall](https://developer.android.com/reference/android/telecom/Call.RttCall.html)
    ///
    /// Required feature: android-telecom-Call_RttCall
    public final class Call_RttCall ("android/telecom/Call$RttCall") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [RttCall](https://developer.android.com/reference/android/telecom/Call.RttCall.html#RttCall())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::telecom::Call_RttCall>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/telecom/Call$RttCall", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getRttAudioMode](https://developer.android.com/reference/android/telecom/Call.RttCall.html#getRttAudioMode())
        pub fn getRttAudioMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/Call$RttCall", java.flags == PUBLIC, .name == "getRttAudioMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "getRttAudioMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRttMode](https://developer.android.com/reference/android/telecom/Call.RttCall.html#setRttMode(int))
        pub fn setRttMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/Call$RttCall", java.flags == PUBLIC, .name == "setRttMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "setRttMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/android/telecom/Call.RttCall.html#write(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn write<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/Call$RttCall", java.flags == PUBLIC, .name == "write", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "write\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/android/telecom/Call.RttCall.html#read())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/Call$RttCall", java.flags == PUBLIC, .name == "read", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "read\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readImmediately](https://developer.android.com/reference/android/telecom/Call.RttCall.html#readImmediately())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn readImmediately<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/telecom/Call$RttCall", java.flags == PUBLIC, .name == "readImmediately", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/telecom/Call$RttCall\0", "readImmediately\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [RTT_MODE_FULL](https://developer.android.com/reference/android/telecom/Call.RttCall.html#RTT_MODE_FULL)
        pub const RTT_MODE_FULL : i32 = 1;

        /// public static final [RTT_MODE_HCO](https://developer.android.com/reference/android/telecom/Call.RttCall.html#RTT_MODE_HCO)
        pub const RTT_MODE_HCO : i32 = 2;

        /// public static final [RTT_MODE_VCO](https://developer.android.com/reference/android/telecom/Call.RttCall.html#RTT_MODE_VCO)
        pub const RTT_MODE_VCO : i32 = 3;
    }
}
