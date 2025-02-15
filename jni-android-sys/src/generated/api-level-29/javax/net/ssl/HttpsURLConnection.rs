// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-HttpsURLConnection"))]
__jni_bindgen! {
    /// public class [HttpsURLConnection](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html)
    ///
    /// Required feature: javax-net-ssl-HttpsURLConnection
    public class HttpsURLConnection ("javax/net/ssl/HttpsURLConnection") extends crate::java::net::HttpURLConnection {

        // // Not emitting: Non-public method
        // /// [HttpsURLConnection](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#HttpsURLConnection(java.net.URL))
        // ///
        // /// Required features: "java-net-URL"
        // #[cfg(any(feature = "all", all(feature = "java-net-URL")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URL>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::HttpsURLConnection>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/net/URL;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "<init>\0", "(Ljava/net/URL;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getCipherSuite](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getCipherSuite())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getCipherSuite<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | ABSTRACT, .name == "getCipherSuite", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getCipherSuite\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalCertificates](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getLocalCertificates())
        ///
        /// Required features: "java-security-cert-Certificate"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-Certificate")))]
        pub fn getLocalCertificates<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::Certificate, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | ABSTRACT, .name == "getLocalCertificates", .descriptor == "()[Ljava/security/cert/Certificate;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getLocalCertificates\0", "()[Ljava/security/cert/Certificate;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getServerCertificates](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getServerCertificates())
        ///
        /// Required features: "java-security-cert-Certificate"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-Certificate")))]
        pub fn getServerCertificates<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::cert::Certificate, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | ABSTRACT, .name == "getServerCertificates", .descriptor == "()[Ljava/security/cert/Certificate;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getServerCertificates\0", "()[Ljava/security/cert/Certificate;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPeerPrincipal](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getPeerPrincipal())
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn getPeerPrincipal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "getPeerPrincipal", .descriptor == "()Ljava/security/Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getPeerPrincipal\0", "()Ljava/security/Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLocalPrincipal](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getLocalPrincipal())
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn getLocalPrincipal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "getLocalPrincipal", .descriptor == "()Ljava/security/Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getLocalPrincipal\0", "()Ljava/security/Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDefaultHostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#setDefaultHostnameVerifier(javax.net.ssl.HostnameVerifier))
        ///
        /// Required features: "javax-net-ssl-HostnameVerifier"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HostnameVerifier")))]
        pub fn setDefaultHostnameVerifier<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::HostnameVerifier>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | STATIC, .name == "setDefaultHostnameVerifier", .descriptor == "(Ljavax/net/ssl/HostnameVerifier;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/HttpsURLConnection\0", "setDefaultHostnameVerifier\0", "(Ljavax/net/ssl/HostnameVerifier;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultHostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getDefaultHostnameVerifier())
        ///
        /// Required features: "javax-net-ssl-HostnameVerifier"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HostnameVerifier")))]
        pub fn getDefaultHostnameVerifier<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::HostnameVerifier>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | STATIC, .name == "getDefaultHostnameVerifier", .descriptor == "()Ljavax/net/ssl/HostnameVerifier;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/HttpsURLConnection\0", "getDefaultHostnameVerifier\0", "()Ljavax/net/ssl/HostnameVerifier;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setHostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#setHostnameVerifier(javax.net.ssl.HostnameVerifier))
        ///
        /// Required features: "javax-net-ssl-HostnameVerifier"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HostnameVerifier")))]
        pub fn setHostnameVerifier<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::HostnameVerifier>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "setHostnameVerifier", .descriptor == "(Ljavax/net/ssl/HostnameVerifier;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "setHostnameVerifier\0", "(Ljavax/net/ssl/HostnameVerifier;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getHostnameVerifier())
        ///
        /// Required features: "javax-net-ssl-HostnameVerifier"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-HostnameVerifier")))]
        pub fn getHostnameVerifier<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::HostnameVerifier>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "getHostnameVerifier", .descriptor == "()Ljavax/net/ssl/HostnameVerifier;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getHostnameVerifier\0", "()Ljavax/net/ssl/HostnameVerifier;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDefaultSSLSocketFactory](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#setDefaultSSLSocketFactory(javax.net.ssl.SSLSocketFactory))
        ///
        /// Required features: "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSocketFactory")))]
        pub fn setDefaultSSLSocketFactory<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLSocketFactory>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | STATIC, .name == "setDefaultSSLSocketFactory", .descriptor == "(Ljavax/net/ssl/SSLSocketFactory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/HttpsURLConnection\0", "setDefaultSSLSocketFactory\0", "(Ljavax/net/ssl/SSLSocketFactory;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefaultSSLSocketFactory](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getDefaultSSLSocketFactory())
        ///
        /// Required features: "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSocketFactory")))]
        pub fn getDefaultSSLSocketFactory<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocketFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC | STATIC, .name == "getDefaultSSLSocketFactory", .descriptor == "()Ljavax/net/ssl/SSLSocketFactory;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/HttpsURLConnection\0", "getDefaultSSLSocketFactory\0", "()Ljavax/net/ssl/SSLSocketFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSSLSocketFactory](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#setSSLSocketFactory(javax.net.ssl.SSLSocketFactory))
        ///
        /// Required features: "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSocketFactory")))]
        pub fn setSSLSocketFactory<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLSocketFactory>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "setSSLSocketFactory", .descriptor == "(Ljavax/net/ssl/SSLSocketFactory;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "setSSLSocketFactory\0", "(Ljavax/net/ssl/SSLSocketFactory;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSSLSocketFactory](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#getSSLSocketFactory())
        ///
        /// Required features: "javax-net-ssl-SSLSocketFactory"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSocketFactory")))]
        pub fn getSSLSocketFactory<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSocketFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/HttpsURLConnection", java.flags == PUBLIC, .name == "getSSLSocketFactory", .descriptor == "()Ljavax/net/ssl/SSLSocketFactory;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/HttpsURLConnection\0", "getSSLSocketFactory\0", "()Ljavax/net/ssl/SSLSocketFactory;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected [hostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#hostnameVerifier)
        // ///
        // /// Required feature: javax-net-ssl-HostnameVerifier
        // #[cfg(any(feature = "all", feature = "javax-net-ssl-HostnameVerifier"))]
        // pub fn hostnameVerifier<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::HostnameVerifier>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("javax/net/ssl/HttpsURLConnection\0", "hostnameVerifier\0", "Ljavax/net/ssl/HostnameVerifier;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [hostnameVerifier](https://developer.android.com/reference/javax/net/ssl/HttpsURLConnection.html#hostnameVerifier)
        // ///
        // /// Required feature: javax-net-ssl-HostnameVerifier
        // #[cfg(any(feature = "all", feature = "javax-net-ssl-HostnameVerifier"))]
        // pub fn set_hostnameVerifier<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::javax::net::ssl::HostnameVerifier>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("javax/net/ssl/HttpsURLConnection\0", "hostnameVerifier\0", "Ljavax/net/ssl/HostnameVerifier;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
