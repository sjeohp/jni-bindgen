// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-MembershipKey"))]
__jni_bindgen! {
    /// public class [MembershipKey](https://developer.android.com/reference/java/nio/channels/MembershipKey.html)
    ///
    /// Required feature: java-nio-channels-MembershipKey
    public class MembershipKey ("java/nio/channels/MembershipKey") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [MembershipKey](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#MembershipKey())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::channels::MembershipKey>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/channels/MembershipKey", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isValid](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#isValid())
        pub fn isValid<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "isValid", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "isValid\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [drop](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#drop())
        pub fn drop<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "drop", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "drop\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [block](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#block(java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress", "java-nio-channels-MembershipKey"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress", feature = "java-nio-channels-MembershipKey")))]
        pub fn block<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::MembershipKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "block", .descriptor == "(Ljava/net/InetAddress;)Ljava/nio/channels/MembershipKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "block\0", "(Ljava/net/InetAddress;)Ljava/nio/channels/MembershipKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [unblock](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#unblock(java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress", "java-nio-channels-MembershipKey"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress", feature = "java-nio-channels-MembershipKey")))]
        pub fn unblock<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::MembershipKey>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "unblock", .descriptor == "(Ljava/net/InetAddress;)Ljava/nio/channels/MembershipKey;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "unblock\0", "(Ljava/net/InetAddress;)Ljava/nio/channels/MembershipKey;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [channel](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#channel())
        ///
        /// Required features: "java-nio-channels-MulticastChannel"
        #[cfg(any(feature = "all", all(feature = "java-nio-channels-MulticastChannel")))]
        pub fn channel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::MulticastChannel>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "channel", .descriptor == "()Ljava/nio/channels/MulticastChannel;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "channel\0", "()Ljava/nio/channels/MulticastChannel;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [group](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#group())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn group<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "group", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "group\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [networkInterface](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#networkInterface())
        ///
        /// Required features: "java-net-NetworkInterface"
        #[cfg(any(feature = "all", all(feature = "java-net-NetworkInterface")))]
        pub fn networkInterface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::NetworkInterface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "networkInterface", .descriptor == "()Ljava/net/NetworkInterface;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "networkInterface\0", "()Ljava/net/NetworkInterface;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [sourceAddress](https://developer.android.com/reference/java/nio/channels/MembershipKey.html#sourceAddress())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn sourceAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/MembershipKey", java.flags == PUBLIC | ABSTRACT, .name == "sourceAddress", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/MembershipKey\0", "sourceAddress\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
