// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-SSLEngineResult"))]
__jni_bindgen! {
    /// public class [SSLEngineResult](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html)
    ///
    /// Required feature: javax-net-ssl-SSLEngineResult
    public class SSLEngineResult ("javax/net/ssl/SSLEngineResult") extends crate::java::lang::Object {

        /// [SSLEngineResult](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#SSLEngineResult(javax.net.ssl.SSLEngineResult.Status,%20javax.net.ssl.SSLEngineResult.HandshakeStatus,%20int,%20int))
        ///
        /// Required features: "javax-net-ssl-SSLEngineResult_HandshakeStatus", "javax-net-ssl-SSLEngineResult_Status"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus", feature = "javax-net-ssl-SSLEngineResult_Status")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLEngineResult_Status>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/net/ssl/SSLEngineResult$Status;Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "<init>\0", "(Ljavax/net/ssl/SSLEngineResult$Status;Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;II)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStatus](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#getStatus())
        ///
        /// Required features: "javax-net-ssl-SSLEngineResult_Status"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLEngineResult_Status")))]
        pub fn getStatus<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_Status>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC | FINAL, .name == "getStatus", .descriptor == "()Ljavax/net/ssl/SSLEngineResult$Status;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "getStatus\0", "()Ljavax/net/ssl/SSLEngineResult$Status;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHandshakeStatus](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#getHandshakeStatus())
        ///
        /// Required features: "javax-net-ssl-SSLEngineResult_HandshakeStatus"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLEngineResult_HandshakeStatus")))]
        pub fn getHandshakeStatus<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLEngineResult_HandshakeStatus>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC | FINAL, .name == "getHandshakeStatus", .descriptor == "()Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "getHandshakeStatus\0", "()Ljavax/net/ssl/SSLEngineResult$HandshakeStatus;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bytesConsumed](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#bytesConsumed())
        pub fn bytesConsumed<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC | FINAL, .name == "bytesConsumed", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "bytesConsumed\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [bytesProduced](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#bytesProduced())
        pub fn bytesProduced<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC | FINAL, .name == "bytesProduced", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "bytesProduced\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/javax/net/ssl/SSLEngineResult.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLEngineResult", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLEngineResult\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
