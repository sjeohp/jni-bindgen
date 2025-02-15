// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-net-SSLCertificateSocketFactory"))]
__jni_bindgen! {
    /// public class [SSLCertificateSocketFactory](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html)
    ///
    /// Required feature: android-net-SSLCertificateSocketFactory
    #[deprecated] public class SSLCertificateSocketFactory ("android/net/SSLCertificateSocketFactory") extends crate::javax::net::ssl::SSLSocketFactory {

        /// [SSLCertificateSocketFactory](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#SSLCertificateSocketFactory(int))
        #[deprecated] pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::net::SSLCertificateSocketFactory>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefault](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getDefault(int))
        ///
        /// Required features: "javax-net-SocketFactory"
        #[cfg(any(feature = "all", all(feature = "javax-net-SocketFactory")))]
        #[deprecated] pub fn getDefault_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::SocketFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC | STATIC, .name == "getDefault", .descriptor == "(I)Ljavax/net/SocketFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/SSLCertificateSocketFactory\0", "getDefault\0", "(I)Ljavax/net/SocketFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefault](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getDefault(int,%20android.net.SSLSessionCache))
        ///
        /// Required features: "android-net-SSLSessionCache", "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "android-net-SSLSessionCache", feature = "javax-net-ssl-SSLSocketFactory")))]
        #[deprecated] pub fn getDefault_int_SSLSessionCache<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::SSLSessionCache>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocketFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC | STATIC, .name == "getDefault", .descriptor == "(ILandroid/net/SSLSessionCache;)Ljavax/net/ssl/SSLSocketFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/SSLCertificateSocketFactory\0", "getDefault\0", "(ILandroid/net/SSLSessionCache;)Ljavax/net/ssl/SSLSocketFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInsecure](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getInsecure(int,%20android.net.SSLSessionCache))
        ///
        /// Required features: "android-net-SSLSessionCache", "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "android-net-SSLSessionCache", feature = "javax-net-ssl-SSLSocketFactory")))]
        #[deprecated] pub fn getInsecure<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::android::net::SSLSessionCache>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocketFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC | STATIC, .name == "getInsecure", .descriptor == "(ILandroid/net/SSLSessionCache;)Ljavax/net/ssl/SSLSocketFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("android/net/SSLCertificateSocketFactory\0", "getInsecure\0", "(ILandroid/net/SSLSessionCache;)Ljavax/net/ssl/SSLSocketFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTrustManagers](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#setTrustManagers(javax.net.ssl.TrustManager%5B%5D))
        ///
        /// Required features: "javax-net-ssl-TrustManager"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-TrustManager")))]
        #[deprecated] pub fn setTrustManagers<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::javax::net::ssl::TrustManager, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "setTrustManagers", .descriptor == "([Ljavax/net/ssl/TrustManager;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "setTrustManagers\0", "([Ljavax/net/ssl/TrustManager;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setNpnProtocols](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#setNpnProtocols(byte%5B%5D%5B%5D))
        #[deprecated] pub fn setNpnProtocols<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<__jni_bindgen::ByteArray, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "setNpnProtocols", .descriptor == "([[B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "setNpnProtocols\0", "([[B)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNpnSelectedProtocol](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getNpnSelectedProtocol(java.net.Socket))
        ///
        /// Required features: "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-Socket")))]
        #[deprecated] pub fn getNpnSelectedProtocol<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "getNpnSelectedProtocol", .descriptor == "(Ljava/net/Socket;)[B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "getNpnSelectedProtocol\0", "(Ljava/net/Socket;)[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setKeyManagers](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#setKeyManagers(javax.net.ssl.KeyManager%5B%5D))
        ///
        /// Required features: "javax-net-ssl-KeyManager"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-KeyManager")))]
        #[deprecated] pub fn setKeyManagers<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::javax::net::ssl::KeyManager, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "setKeyManagers", .descriptor == "([Ljavax/net/ssl/KeyManager;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "setKeyManagers\0", "([Ljavax/net/ssl/KeyManager;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setUseSessionTickets](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#setUseSessionTickets(java.net.Socket,%20boolean))
        ///
        /// Required features: "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-Socket")))]
        #[deprecated] pub fn setUseSessionTickets<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>, arg1: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "setUseSessionTickets", .descriptor == "(Ljava/net/Socket;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "setUseSessionTickets\0", "(Ljava/net/Socket;Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHostname](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#setHostname(java.net.Socket,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-Socket")))]
        #[deprecated] pub fn setHostname<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "setHostname", .descriptor == "(Ljava/net/Socket;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "setHostname\0", "(Ljava/net/Socket;Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket(java.net.Socket,%20java.lang.String,%20int,%20boolean))
        ///
        /// Required features: "java-lang-String", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket_Socket_String_int_boolean<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32, arg3: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "(Ljava/net/Socket;Ljava/lang/String;IZ)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "(Ljava/net/Socket;Ljava/lang/String;IZ)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket())
        ///
        /// Required features: "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "()Ljava/net/Socket;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "()Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket(java.net.InetAddress,%20int,%20java.net.InetAddress,%20int))
        ///
        /// Required features: "java-net-InetAddress", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress", feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket_InetAddress_int_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "(Ljava/net/InetAddress;ILjava/net/InetAddress;I)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "(Ljava/net/InetAddress;ILjava/net/InetAddress;I)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket(java.net.InetAddress,%20int))
        ///
        /// Required features: "java-net-InetAddress", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-InetAddress", feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "(Ljava/net/InetAddress;I)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "(Ljava/net/InetAddress;I)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket(java.lang.String,%20int,%20java.net.InetAddress,%20int))
        ///
        /// Required features: "java-lang-String", "java-net-InetAddress", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-InetAddress", feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket_String_int_InetAddress_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "(Ljava/lang/String;ILjava/net/InetAddress;I)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into()), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "(Ljava/lang/String;ILjava/net/InetAddress;I)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [createSocket](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#createSocket(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String", "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket_String_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "createSocket", .descriptor == "(Ljava/lang/String;I)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "createSocket\0", "(Ljava/lang/String;I)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultCipherSuites](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getDefaultCipherSuites())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn getDefaultCipherSuites<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "getDefaultCipherSuites", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "getDefaultCipherSuites\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSupportedCipherSuites](https://developer.android.com/reference/android/net/SSLCertificateSocketFactory.html#getSupportedCipherSuites())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        #[deprecated] pub fn getSupportedCipherSuites<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/net/SSLCertificateSocketFactory", java.flags == PUBLIC, .name == "getSupportedCipherSuites", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/net/SSLCertificateSocketFactory\0", "getSupportedCipherSuites\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
