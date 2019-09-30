// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-MulticastSocket"))]
__jni_bindgen! {
    /// public class [MulticastSocket](https://developer.android.com/reference/java/net/MulticastSocket.html)
    ///
    /// Required feature: java-net-MulticastSocket
    public class MulticastSocket ("java/net/MulticastSocket") extends crate::java::net::DatagramSocket {

        /// [MulticastSocket](https://developer.android.com/reference/java/net/MulticastSocket.html#MulticastSocket())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::MulticastSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [MulticastSocket](https://developer.android.com/reference/java/net/MulticastSocket.html#MulticastSocket(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::MulticastSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [MulticastSocket](https://developer.android.com/reference/java/net/MulticastSocket.html#MulticastSocket(java.net.SocketAddress))
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn new_SocketAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::MulticastSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/net/SocketAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "<init>\0", "(Ljava/net/SocketAddress;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTTL](https://developer.android.com/reference/java/net/MulticastSocket.html#setTTL(byte))
        #[deprecated] pub fn setTTL<'env>(&'env self, arg0: i8) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "setTTL", .descriptor == "(B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "setTTL\0", "(B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTimeToLive](https://developer.android.com/reference/java/net/MulticastSocket.html#setTimeToLive(int))
        pub fn setTimeToLive<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "setTimeToLive", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "setTimeToLive\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTTL](https://developer.android.com/reference/java/net/MulticastSocket.html#getTTL())
        #[deprecated] pub fn getTTL<'env>(&'env self) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "getTTL", .descriptor == "()B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "getTTL\0", "()B\0");
                __jni_env.call_byte_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimeToLive](https://developer.android.com/reference/java/net/MulticastSocket.html#getTimeToLive())
        pub fn getTimeToLive<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "getTimeToLive", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "getTimeToLive\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [joinGroup](https://developer.android.com/reference/java/net/MulticastSocket.html#joinGroup(java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn joinGroup_InetAddress<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "joinGroup", .descriptor == "(Ljava/net/InetAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "joinGroup\0", "(Ljava/net/InetAddress;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [leaveGroup](https://developer.android.com/reference/java/net/MulticastSocket.html#leaveGroup(java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn leaveGroup_InetAddress<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "leaveGroup", .descriptor == "(Ljava/net/InetAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "leaveGroup\0", "(Ljava/net/InetAddress;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [joinGroup](https://developer.android.com/reference/java/net/MulticastSocket.html#joinGroup(java.net.SocketAddress,%20java.net.NetworkInterface))
        ///
        /// Required features: "java-net-NetworkInterface", "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-NetworkInterface", feature = "java-net-SocketAddress")))]
        pub fn joinGroup_SocketAddress_NetworkInterface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::NetworkInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "joinGroup", .descriptor == "(Ljava/net/SocketAddress;Ljava/net/NetworkInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "joinGroup\0", "(Ljava/net/SocketAddress;Ljava/net/NetworkInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [leaveGroup](https://developer.android.com/reference/java/net/MulticastSocket.html#leaveGroup(java.net.SocketAddress,%20java.net.NetworkInterface))
        ///
        /// Required features: "java-net-NetworkInterface", "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-NetworkInterface", feature = "java-net-SocketAddress")))]
        pub fn leaveGroup_SocketAddress_NetworkInterface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::NetworkInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "leaveGroup", .descriptor == "(Ljava/net/SocketAddress;Ljava/net/NetworkInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "leaveGroup\0", "(Ljava/net/SocketAddress;Ljava/net/NetworkInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInterface](https://developer.android.com/reference/java/net/MulticastSocket.html#setInterface(java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn setInterface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "setInterface", .descriptor == "(Ljava/net/InetAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "setInterface\0", "(Ljava/net/InetAddress;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInterface](https://developer.android.com/reference/java/net/MulticastSocket.html#getInterface())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn getInterface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "getInterface", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "getInterface\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNetworkInterface](https://developer.android.com/reference/java/net/MulticastSocket.html#setNetworkInterface(java.net.NetworkInterface))
        ///
        /// Required features: "java-net-NetworkInterface"
        #[cfg(any(feature = "all", all(feature = "java-net-NetworkInterface")))]
        pub fn setNetworkInterface<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::NetworkInterface>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "setNetworkInterface", .descriptor == "(Ljava/net/NetworkInterface;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "setNetworkInterface\0", "(Ljava/net/NetworkInterface;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNetworkInterface](https://developer.android.com/reference/java/net/MulticastSocket.html#getNetworkInterface())
        ///
        /// Required features: "java-net-NetworkInterface"
        #[cfg(any(feature = "all", all(feature = "java-net-NetworkInterface")))]
        pub fn getNetworkInterface<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::NetworkInterface>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "getNetworkInterface", .descriptor == "()Ljava/net/NetworkInterface;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "getNetworkInterface\0", "()Ljava/net/NetworkInterface;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLoopbackMode](https://developer.android.com/reference/java/net/MulticastSocket.html#setLoopbackMode(boolean))
        pub fn setLoopbackMode<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "setLoopbackMode", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "setLoopbackMode\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLoopbackMode](https://developer.android.com/reference/java/net/MulticastSocket.html#getLoopbackMode())
        pub fn getLoopbackMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "getLoopbackMode", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "getLoopbackMode\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [send](https://developer.android.com/reference/java/net/MulticastSocket.html#send(java.net.DatagramPacket,%20byte))
        ///
        /// Required features: "java-net-DatagramPacket"
        #[cfg(any(feature = "all", all(feature = "java-net-DatagramPacket")))]
        #[deprecated] pub fn send<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramPacket>>, arg1: i8) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/MulticastSocket", java.flags == PUBLIC, .name == "send", .descriptor == "(Ljava/net/DatagramPacket;B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/MulticastSocket\0", "send\0", "(Ljava/net/DatagramPacket;B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
