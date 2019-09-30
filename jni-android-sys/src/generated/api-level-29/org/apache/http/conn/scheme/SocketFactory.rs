// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "org-apache-http-conn-scheme-SocketFactory"))]
__jni_bindgen! {
    /// public interface [SocketFactory](https://developer.android.com/reference/org/apache/http/conn/scheme/SocketFactory.html)
    ///
    /// Required feature: org-apache-http-conn-scheme-SocketFactory
    #[deprecated] public interface SocketFactory ("org/apache/http/conn/scheme/SocketFactory") extends crate::java::lang::Object {

        /// [createSocket](https://developer.android.com/reference/org/apache/http/conn/scheme/SocketFactory.html#createSocket())
        ///
        /// Required features: "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-Socket")))]
        #[deprecated] pub fn createSocket<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/apache/http/conn/scheme/SocketFactory", java.flags == PUBLIC | ABSTRACT, .name == "createSocket", .descriptor == "()Ljava/net/Socket;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/apache/http/conn/scheme/SocketFactory\0", "createSocket\0", "()Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connectSocket](https://developer.android.com/reference/org/apache/http/conn/scheme/SocketFactory.html#connectSocket(java.net.Socket,%20java.lang.String,%20int,%20java.net.InetAddress,%20int,%20org.apache.http.params.HttpParams))
        ///
        /// Required features: "java-lang-String", "java-net-InetAddress", "java-net-Socket", "org-apache-http-params-HttpParams"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-net-InetAddress", feature = "java-net-Socket", feature = "org-apache-http-params-HttpParams")))]
        #[deprecated] pub fn connectSocket<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32, arg3: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::InetAddress>>, arg4: i32, arg5: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::org::apache::http::params::HttpParams>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Socket>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/apache/http/conn/scheme/SocketFactory", java.flags == PUBLIC | ABSTRACT, .name == "connectSocket", .descriptor == "(Ljava/net/Socket;Ljava/lang/String;ILjava/net/InetAddress;ILorg/apache/http/params/HttpParams;)Ljava/net/Socket;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3.into()), __jni_bindgen::AsJValue::as_jvalue(&arg4), __jni_bindgen::AsJValue::as_jvalue(&arg5.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/apache/http/conn/scheme/SocketFactory\0", "connectSocket\0", "(Ljava/net/Socket;Ljava/lang/String;ILjava/net/InetAddress;ILorg/apache/http/params/HttpParams;)Ljava/net/Socket;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isSecure](https://developer.android.com/reference/org/apache/http/conn/scheme/SocketFactory.html#isSecure(java.net.Socket))
        ///
        /// Required features: "java-net-Socket"
        #[cfg(any(feature = "all", all(feature = "java-net-Socket")))]
        #[deprecated] pub fn isSecure<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Socket>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "org/apache/http/conn/scheme/SocketFactory", java.flags == PUBLIC | ABSTRACT, .name == "isSecure", .descriptor == "(Ljava/net/Socket;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("org/apache/http/conn/scheme/SocketFactory\0", "isSecure\0", "(Ljava/net/Socket;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
