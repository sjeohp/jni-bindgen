// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-SSLSocket"))]
__jni_bindgen! {
    /// public class [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html)
    ///
    /// Required feature: javax-net-ssl-SSLSocket
    public class SSLSocket ("javax/net/ssl/SSLSocket") extends crate::java::net::Socket {

        // // Not emitting: Non-public method
        // /// [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#SSLSocket())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#SSLSocket(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#SSLSocket(java.net.InetAddress,%20int))
        // ///
        // /// Required features: "java-net-InetAddress"
        // #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/net/InetAddress;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "<init>\0", "(Ljava/net/InetAddress;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#SSLSocket(java.lang.String,%20int,%20java.net.InetAddress,%20int))
        // ///
        // /// Required features: "java-lang-String", "java-net-InetAddress"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-InetAddress")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;ILjava/net/InetAddress;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "<init>\0", "(Ljava/lang/String;ILjava/net/InetAddress;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [SSLSocket](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#SSLSocket(java.net.InetAddress,%20int,%20java.net.InetAddress,%20int))
        // ///
        // /// Required features: "java-net-InetAddress"
        // #[cfg(any(feature = "all", all(feature = "java-net-InetAddress")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocket>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SSLSocket", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/net/InetAddress;ILjava/net/InetAddress;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "<init>\0", "(Ljava/net/InetAddress;ILjava/net/InetAddress;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getSupportedCipherSuites](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getSupportedCipherSuites())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSupportedCipherSuites<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getSupportedCipherSuites", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getSupportedCipherSuites\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEnabledCipherSuites](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getEnabledCipherSuites())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEnabledCipherSuites<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getEnabledCipherSuites", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getEnabledCipherSuites\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEnabledCipherSuites](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setEnabledCipherSuites(java.lang.String%5B%5D))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setEnabledCipherSuites<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setEnabledCipherSuites", .descriptor == "([Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setEnabledCipherSuites\0", "([Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSupportedProtocols](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getSupportedProtocols())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSupportedProtocols<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getSupportedProtocols", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getSupportedProtocols\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEnabledProtocols](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getEnabledProtocols())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEnabledProtocols<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getEnabledProtocols", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getEnabledProtocols\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEnabledProtocols](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setEnabledProtocols(java.lang.String%5B%5D))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setEnabledProtocols<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setEnabledProtocols", .descriptor == "([Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setEnabledProtocols\0", "([Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSession](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getSession())
        ///
        /// Required features: "javax-net-ssl-SSLSession"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSession")))]
        pub fn getSession<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSession>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getSession", .descriptor == "()Ljavax/net/ssl/SSLSession;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getSession\0", "()Ljavax/net/ssl/SSLSession;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHandshakeSession](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getHandshakeSession())
        ///
        /// Required features: "javax-net-ssl-SSLSession"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSession")))]
        pub fn getHandshakeSession<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSession>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "getHandshakeSession", .descriptor == "()Ljavax/net/ssl/SSLSession;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getHandshakeSession\0", "()Ljavax/net/ssl/SSLSession;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addHandshakeCompletedListener](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#addHandshakeCompletedListener(javax.net.ssl.HandshakeCompletedListener))
        ///
        /// Required features: "javax-net-ssl-HandshakeCompletedListener"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HandshakeCompletedListener")))]
        pub fn addHandshakeCompletedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::HandshakeCompletedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "addHandshakeCompletedListener", .descriptor == "(Ljavax/net/ssl/HandshakeCompletedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "addHandshakeCompletedListener\0", "(Ljavax/net/ssl/HandshakeCompletedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeHandshakeCompletedListener](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#removeHandshakeCompletedListener(javax.net.ssl.HandshakeCompletedListener))
        ///
        /// Required features: "javax-net-ssl-HandshakeCompletedListener"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HandshakeCompletedListener")))]
        pub fn removeHandshakeCompletedListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::HandshakeCompletedListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "removeHandshakeCompletedListener", .descriptor == "(Ljavax/net/ssl/HandshakeCompletedListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "removeHandshakeCompletedListener\0", "(Ljavax/net/ssl/HandshakeCompletedListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [startHandshake](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#startHandshake())
        pub fn startHandshake<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "startHandshake", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "startHandshake\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setUseClientMode](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setUseClientMode(boolean))
        pub fn setUseClientMode<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setUseClientMode", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setUseClientMode\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getUseClientMode](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getUseClientMode())
        pub fn getUseClientMode<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getUseClientMode", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getUseClientMode\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNeedClientAuth](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setNeedClientAuth(boolean))
        pub fn setNeedClientAuth<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setNeedClientAuth", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setNeedClientAuth\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNeedClientAuth](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getNeedClientAuth())
        pub fn getNeedClientAuth<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getNeedClientAuth", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getNeedClientAuth\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setWantClientAuth](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setWantClientAuth(boolean))
        pub fn setWantClientAuth<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setWantClientAuth", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setWantClientAuth\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getWantClientAuth](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getWantClientAuth())
        pub fn getWantClientAuth<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getWantClientAuth", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getWantClientAuth\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setEnableSessionCreation](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setEnableSessionCreation(boolean))
        pub fn setEnableSessionCreation<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "setEnableSessionCreation", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setEnableSessionCreation\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEnableSessionCreation](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getEnableSessionCreation())
        pub fn getEnableSessionCreation<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC | ABSTRACT, .name == "getEnableSessionCreation", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getEnableSessionCreation\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSSLParameters](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getSSLParameters())
        ///
        /// Required features: "javax-net-ssl-SSLParameters"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLParameters")))]
        pub fn getSSLParameters<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLParameters>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "getSSLParameters", .descriptor == "()Ljavax/net/ssl/SSLParameters;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getSSLParameters\0", "()Ljavax/net/ssl/SSLParameters;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSSLParameters](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setSSLParameters(javax.net.ssl.SSLParameters))
        ///
        /// Required features: "javax-net-ssl-SSLParameters"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLParameters")))]
        pub fn setSSLParameters<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLParameters>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "setSSLParameters", .descriptor == "(Ljavax/net/ssl/SSLParameters;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setSSLParameters\0", "(Ljavax/net/ssl/SSLParameters;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getApplicationProtocol](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getApplicationProtocol())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getApplicationProtocol<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "getApplicationProtocol", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getApplicationProtocol\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHandshakeApplicationProtocol](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getHandshakeApplicationProtocol())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getHandshakeApplicationProtocol<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "getHandshakeApplicationProtocol", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getHandshakeApplicationProtocol\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHandshakeApplicationProtocolSelector](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#setHandshakeApplicationProtocolSelector(java.util.function.BiFunction))
        ///
        /// Required features: "java-util-function-BiFunction"
        #[cfg(any(feature = "all", all(feature = "java-util-function-BiFunction")))]
        pub fn setHandshakeApplicationProtocolSelector<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::function::BiFunction>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "setHandshakeApplicationProtocolSelector", .descriptor == "(Ljava/util/function/BiFunction;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "setHandshakeApplicationProtocolSelector\0", "(Ljava/util/function/BiFunction;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHandshakeApplicationProtocolSelector](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#getHandshakeApplicationProtocolSelector())
        ///
        /// Required features: "java-util-function-BiFunction"
        #[cfg(any(feature = "all", all(feature = "java-util-function-BiFunction")))]
        pub fn getHandshakeApplicationProtocolSelector<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::function::BiFunction>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "getHandshakeApplicationProtocolSelector", .descriptor == "()Ljava/util/function/BiFunction;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "getHandshakeApplicationProtocolSelector\0", "()Ljava/util/function/BiFunction;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/javax/net/ssl/SSLSocket.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSocket", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSocket\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
