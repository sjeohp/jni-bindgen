// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-rtp-RtpStream"))]
__jni_bindgen! {
    /// public class [RtpStream](https://developer.android.com/reference/android/net/rtp/RtpStream.html)
    ///
    /// Required feature: android-net-rtp-RtpStream
    public class RtpStream ("android/net/rtp/RtpStream") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [RtpStream](https://developer.android.com/reference/android/net/rtp/RtpStream.html#RtpStream(java.net.InetAddress))
        // ///
        // /// Required features: "java-net-InetAddress"
        // #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::rtp::RtpStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/rtp/RtpStream", java.flags == (empty), .name == "<init>", .descriptor == "(Ljava/net/InetAddress;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "<init>\0", "(Ljava/net/InetAddress;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getLocalAddress](https://developer.android.com/reference/android/net/rtp/RtpStream.html#getLocalAddress())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn getLocalAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "getLocalAddress", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "getLocalAddress\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalPort](https://developer.android.com/reference/android/net/rtp/RtpStream.html#getLocalPort())
        pub fn getLocalPort<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "getLocalPort", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "getLocalPort\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRemoteAddress](https://developer.android.com/reference/android/net/rtp/RtpStream.html#getRemoteAddress())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn getRemoteAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "getRemoteAddress", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "getRemoteAddress\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRemotePort](https://developer.android.com/reference/android/net/rtp/RtpStream.html#getRemotePort())
        pub fn getRemotePort<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "getRemotePort", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "getRemotePort\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isBusy](https://developer.android.com/reference/android/net/rtp/RtpStream.html#isBusy())
        pub fn isBusy<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "isBusy", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "isBusy\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMode](https://developer.android.com/reference/android/net/rtp/RtpStream.html#getMode())
        pub fn getMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "getMode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "getMode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMode](https://developer.android.com/reference/android/net/rtp/RtpStream.html#setMode(int))
        pub fn setMode<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "setMode", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "setMode\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [associate](https://developer.android.com/reference/android/net/rtp/RtpStream.html#associate(java.net.InetAddress,%20int))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn associate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "associate", .descriptor == "(Ljava/net/InetAddress;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "associate\0", "(Ljava/net/InetAddress;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [release](https://developer.android.com/reference/android/net/rtp/RtpStream.html#release())
        pub fn release<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/rtp/RtpStream", java.flags == PUBLIC, .name == "release", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "release\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [finalize](https://developer.android.com/reference/android/net/rtp/RtpStream.html#finalize())
        // fn finalize<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "android/net/rtp/RtpStream", java.flags == PROTECTED, .name == "finalize", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/rtp/RtpStream\0", "finalize\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [MODE_NORMAL](https://developer.android.com/reference/android/net/rtp/RtpStream.html#MODE_NORMAL)
        pub const MODE_NORMAL : i32 = 0;

        /// public static final [MODE_RECEIVE_ONLY](https://developer.android.com/reference/android/net/rtp/RtpStream.html#MODE_RECEIVE_ONLY)
        pub const MODE_RECEIVE_ONLY : i32 = 2;

        /// public static final [MODE_SEND_ONLY](https://developer.android.com/reference/android/net/rtp/RtpStream.html#MODE_SEND_ONLY)
        pub const MODE_SEND_ONLY : i32 = 1;
    }
}
