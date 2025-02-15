// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-TrustManagerFactory"))]
__jni_bindgen! {
    /// public class [TrustManagerFactory](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html)
    ///
    /// Required feature: javax-net-ssl-TrustManagerFactory
    public class TrustManagerFactory ("javax/net/ssl/TrustManagerFactory") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [TrustManagerFactory](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#TrustManagerFactory(javax.net.ssl.TrustManagerFactorySpi,%20java.security.Provider,%20java.lang.String))
        // ///
        // /// Required features: "java-lang-String", "java-security-Provider", "javax-net-ssl-TrustManagerFactorySpi"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "javax-net-ssl-TrustManagerFactorySpi")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::TrustManagerFactorySpi>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::TrustManagerFactory>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljavax/net/ssl/TrustManagerFactorySpi;Ljava/security/Provider;Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "<init>\0", "(Ljavax/net/ssl/TrustManagerFactorySpi;Ljava/security/Provider;Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getDefaultAlgorithm](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getDefaultAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getDefaultAlgorithm<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getDefaultAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/TrustManagerFactory\0", "getDefaultAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAlgorithm](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getAlgorithm())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getAlgorithm<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | FINAL, .name == "getAlgorithm", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "getAlgorithm\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getInstance(java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-net-ssl-TrustManagerFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-net-ssl-TrustManagerFactory")))]
        pub fn getInstance_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::TrustManagerFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/TrustManagerFactory\0", "getInstance\0", "(Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getInstance(java.lang.String,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-net-ssl-TrustManagerFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-net-ssl-TrustManagerFactory")))]
        pub fn getInstance_String_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::TrustManagerFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/TrustManagerFactory\0", "getInstance\0", "(Ljava/lang/String;Ljava/lang/String;)Ljavax/net/ssl/TrustManagerFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInstance](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getInstance(java.lang.String,%20java.security.Provider))
        ///
        /// Required features: "java-lang-String", "java-security-Provider", "javax-net-ssl-TrustManagerFactory"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Provider", feature = "javax-net-ssl-TrustManagerFactory")))]
        pub fn getInstance_String_Provider<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::TrustManagerFactory>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | STATIC | FINAL, .name == "getInstance", .descriptor == "(Ljava/lang/String;Ljava/security/Provider;)Ljavax/net/ssl/TrustManagerFactory;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/net/ssl/TrustManagerFactory\0", "getInstance\0", "(Ljava/lang/String;Ljava/security/Provider;)Ljavax/net/ssl/TrustManagerFactory;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getProvider](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getProvider())
        ///
        /// Required features: "java-security-Provider"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider")))]
        pub fn getProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Provider>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | FINAL, .name == "getProvider", .descriptor == "()Ljava/security/Provider;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "getProvider\0", "()Ljava/security/Provider;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [init](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#init(java.security.KeyStore))
        ///
        /// Required features: "java-security-KeyStore"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyStore")))]
        pub fn init_KeyStore<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyStore>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | FINAL, .name == "init", .descriptor == "(Ljava/security/KeyStore;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "init\0", "(Ljava/security/KeyStore;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [init](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#init(javax.net.ssl.ManagerFactoryParameters))
        ///
        /// Required features: "javax-net-ssl-ManagerFactoryParameters"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-ManagerFactoryParameters")))]
        pub fn init_ManagerFactoryParameters<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::ManagerFactoryParameters>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | FINAL, .name == "init", .descriptor == "(Ljavax/net/ssl/ManagerFactoryParameters;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "init\0", "(Ljavax/net/ssl/ManagerFactoryParameters;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTrustManagers](https://developer.android.com/reference/javax/net/ssl/TrustManagerFactory.html#getTrustManagers())
        ///
        /// Required features: "javax-net-ssl-TrustManager"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-TrustManager")))]
        pub fn getTrustManagers<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::javax::net::ssl::TrustManager, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/TrustManagerFactory", java.flags == PUBLIC | FINAL, .name == "getTrustManagers", .descriptor == "()[Ljavax/net/ssl/TrustManager;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/TrustManagerFactory\0", "getTrustManagers\0", "()[Ljavax/net/ssl/TrustManager;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
