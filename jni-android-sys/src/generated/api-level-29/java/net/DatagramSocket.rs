// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-DatagramSocket"))]
__jni_bindgen! {
    /// public class [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html)
    ///
    /// Required feature: java-net-DatagramSocket
    public class DatagramSocket ("java/net/DatagramSocket") extends crate::java::lang::Object, implements crate::java::io::Closeable {

        /// [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html#DatagramSocket())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::DatagramSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html#DatagramSocket(java.net.DatagramSocketImpl))
        // ///
        // /// Required features: "java-net-DatagramSocketImpl"
        // #[cfg(any(feature = "all", all(feature = "java-net-DatagramSocketImpl")))]
        // fn new_DatagramSocketImpl<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramSocketImpl>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::DatagramSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/net/DatagramSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/net/DatagramSocketImpl;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "<init>\0", "(Ljava/net/DatagramSocketImpl;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html#DatagramSocket(java.net.SocketAddress))
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn new_SocketAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::DatagramSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/net/SocketAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "<init>\0", "(Ljava/net/SocketAddress;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html#DatagramSocket(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::DatagramSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DatagramSocket](https://developer.android.com/reference/java/net/DatagramSocket.html#DatagramSocket(int,%20java.net.InetAddress))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn new_int_InetAddress<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::DatagramSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "<init>", .descriptor == "(ILjava/net/InetAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "<init>\0", "(ILjava/net/InetAddress;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bind](https://developer.android.com/reference/java/net/DatagramSocket.html#bind(java.net.SocketAddress))
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn bind<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "bind", .descriptor == "(Ljava/net/SocketAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "bind\0", "(Ljava/net/SocketAddress;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/java/net/DatagramSocket.html#connect(java.net.InetAddress,%20int))
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn connect_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "connect", .descriptor == "(Ljava/net/InetAddress;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "connect\0", "(Ljava/net/InetAddress;I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/java/net/DatagramSocket.html#connect(java.net.SocketAddress))
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn connect_SocketAddress<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "connect", .descriptor == "(Ljava/net/SocketAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "connect\0", "(Ljava/net/SocketAddress;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [disconnect](https://developer.android.com/reference/java/net/DatagramSocket.html#disconnect())
        pub fn disconnect<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "disconnect", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "disconnect\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isBound](https://developer.android.com/reference/java/net/DatagramSocket.html#isBound())
        pub fn isBound<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "isBound", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "isBound\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isConnected](https://developer.android.com/reference/java/net/DatagramSocket.html#isConnected())
        pub fn isConnected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "isConnected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "isConnected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInetAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#getInetAddress())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn getInetAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getInetAddress", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getInetAddress\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPort](https://developer.android.com/reference/java/net/DatagramSocket.html#getPort())
        pub fn getPort<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getPort", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getPort\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getRemoteSocketAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#getRemoteSocketAddress())
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn getRemoteSocketAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::SocketAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getRemoteSocketAddress", .descriptor == "()Ljava/net/SocketAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getRemoteSocketAddress\0", "()Ljava/net/SocketAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalSocketAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#getLocalSocketAddress())
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn getLocalSocketAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::SocketAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getLocalSocketAddress", .descriptor == "()Ljava/net/SocketAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getLocalSocketAddress\0", "()Ljava/net/SocketAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [send](https://developer.android.com/reference/java/net/DatagramSocket.html#send(java.net.DatagramPacket))
        ///
        /// Required features: "java-net-DatagramPacket"
        #[cfg(any(feature = "all", all(feature = "java-net-DatagramPacket")))]
        pub fn send<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramPacket>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "send", .descriptor == "(Ljava/net/DatagramPacket;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "send\0", "(Ljava/net/DatagramPacket;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [receive](https://developer.android.com/reference/java/net/DatagramSocket.html#receive(java.net.DatagramPacket))
        ///
        /// Required features: "java-net-DatagramPacket"
        #[cfg(any(feature = "all", all(feature = "java-net-DatagramPacket")))]
        pub fn receive<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramPacket>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "receive", .descriptor == "(Ljava/net/DatagramPacket;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "receive\0", "(Ljava/net/DatagramPacket;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#getLocalAddress())
        ///
        /// Required features: "java-net-InetAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        pub fn getLocalAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::InetAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getLocalAddress", .descriptor == "()Ljava/net/InetAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getLocalAddress\0", "()Ljava/net/InetAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalPort](https://developer.android.com/reference/java/net/DatagramSocket.html#getLocalPort())
        pub fn getLocalPort<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getLocalPort", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getLocalPort\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSoTimeout](https://developer.android.com/reference/java/net/DatagramSocket.html#setSoTimeout(int))
        pub fn setSoTimeout<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setSoTimeout", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setSoTimeout\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSoTimeout](https://developer.android.com/reference/java/net/DatagramSocket.html#getSoTimeout())
        pub fn getSoTimeout<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getSoTimeout", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getSoTimeout\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSendBufferSize](https://developer.android.com/reference/java/net/DatagramSocket.html#setSendBufferSize(int))
        pub fn setSendBufferSize<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setSendBufferSize", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setSendBufferSize\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSendBufferSize](https://developer.android.com/reference/java/net/DatagramSocket.html#getSendBufferSize())
        pub fn getSendBufferSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getSendBufferSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getSendBufferSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setReceiveBufferSize](https://developer.android.com/reference/java/net/DatagramSocket.html#setReceiveBufferSize(int))
        pub fn setReceiveBufferSize<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setReceiveBufferSize", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setReceiveBufferSize\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReceiveBufferSize](https://developer.android.com/reference/java/net/DatagramSocket.html#getReceiveBufferSize())
        pub fn getReceiveBufferSize<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getReceiveBufferSize", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getReceiveBufferSize\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setReuseAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#setReuseAddress(boolean))
        pub fn setReuseAddress<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setReuseAddress", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setReuseAddress\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getReuseAddress](https://developer.android.com/reference/java/net/DatagramSocket.html#getReuseAddress())
        pub fn getReuseAddress<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getReuseAddress", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getReuseAddress\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBroadcast](https://developer.android.com/reference/java/net/DatagramSocket.html#setBroadcast(boolean))
        pub fn setBroadcast<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setBroadcast", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setBroadcast\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBroadcast](https://developer.android.com/reference/java/net/DatagramSocket.html#getBroadcast())
        pub fn getBroadcast<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getBroadcast", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getBroadcast\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTrafficClass](https://developer.android.com/reference/java/net/DatagramSocket.html#setTrafficClass(int))
        pub fn setTrafficClass<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "setTrafficClass", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "setTrafficClass\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTrafficClass](https://developer.android.com/reference/java/net/DatagramSocket.html#getTrafficClass())
        pub fn getTrafficClass<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | SYNCRONIZED, .name == "getTrafficClass", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getTrafficClass\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/net/DatagramSocket.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isClosed](https://developer.android.com/reference/java/net/DatagramSocket.html#isClosed())
        pub fn isClosed<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "isClosed", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "isClosed\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChannel](https://developer.android.com/reference/java/net/DatagramSocket.html#getChannel())
        ///
        /// Required features: "java-nio-channels-DatagramChannel"
        #[cfg(any(feature = "all", all(feature = "java-nio-channels-DatagramChannel")))]
        pub fn getChannel<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::DatagramChannel>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC, .name == "getChannel", .descriptor == "()Ljava/nio/channels/DatagramChannel;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/DatagramSocket\0", "getChannel\0", "()Ljava/nio/channels/DatagramChannel;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDatagramSocketImplFactory](https://developer.android.com/reference/java/net/DatagramSocket.html#setDatagramSocketImplFactory(java.net.DatagramSocketImplFactory))
        ///
        /// Required features: "java-net-DatagramSocketImplFactory"
        #[cfg(any(feature = "all", all(feature = "java-net-DatagramSocketImplFactory")))]
        pub fn setDatagramSocketImplFactory<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::DatagramSocketImplFactory>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/DatagramSocket", java.flags == PUBLIC | STATIC | SYNCRONIZED, .name == "setDatagramSocketImplFactory", .descriptor == "(Ljava/net/DatagramSocketImplFactory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/DatagramSocket\0", "setDatagramSocketImplFactory\0", "(Ljava/net/DatagramSocketImplFactory;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
