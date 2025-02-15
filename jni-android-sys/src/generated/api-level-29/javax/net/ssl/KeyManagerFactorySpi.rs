// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-KeyManagerFactorySpi"))]
__jni_bindgen! {
    /// public class [KeyManagerFactorySpi](https://developer.android.com/reference/javax/net/ssl/KeyManagerFactorySpi.html)
    ///
    /// Required feature: javax-net-ssl-KeyManagerFactorySpi
    public class KeyManagerFactorySpi ("javax/net/ssl/KeyManagerFactorySpi") extends crate::java::lang::Object {

        /// [KeyManagerFactorySpi](https://developer.android.com/reference/javax/net/ssl/KeyManagerFactorySpi.html#KeyManagerFactorySpi())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::KeyManagerFactorySpi>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/KeyManagerFactorySpi", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyManagerFactorySpi\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [engineInit](https://developer.android.com/reference/javax/net/ssl/KeyManagerFactorySpi.html#engineInit(java.security.KeyStore,%20char%5B%5D))
        // ///
        // /// Required features: "java-security-KeyStore"
        // #[cfg(any(feature = "all", all(feature = "java-security-KeyStore")))]
        // fn engineInit<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyStore>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/KeyManagerFactorySpi", java.flags == PROTECTED | ABSTRACT, .name == "engineInit", .descriptor == "(Ljava/security/KeyStore;[C)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyManagerFactorySpi\0", "engineInit\0", "(Ljava/security/KeyStore;[C)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineInit](https://developer.android.com/reference/javax/net/ssl/KeyManagerFactorySpi.html#engineInit(javax.net.ssl.ManagerFactoryParameters))
        // ///
        // /// Required features: "javax-net-ssl-ManagerFactoryParameters"
        // #[cfg(any(feature = "all", all(feature = "javax-net-ssl-ManagerFactoryParameters")))]
        // fn engineInit<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::ManagerFactoryParameters>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/KeyManagerFactorySpi", java.flags == PROTECTED | ABSTRACT, .name == "engineInit", .descriptor == "(Ljavax/net/ssl/ManagerFactoryParameters;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyManagerFactorySpi\0", "engineInit\0", "(Ljavax/net/ssl/ManagerFactoryParameters;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineGetKeyManagers](https://developer.android.com/reference/javax/net/ssl/KeyManagerFactorySpi.html#engineGetKeyManagers())
        // ///
        // /// Required features: "javax-net-ssl-KeyManager"
        // #[cfg(any(feature = "all", all(feature = "javax-net-ssl-KeyManager")))]
        // fn engineGetKeyManagers<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::javax::net::ssl::KeyManager, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/KeyManagerFactorySpi", java.flags == PROTECTED | ABSTRACT, .name == "engineGetKeyManagers", .descriptor == "()[Ljavax/net/ssl/KeyManager;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyManagerFactorySpi\0", "engineGetKeyManagers\0", "()[Ljavax/net/ssl/KeyManager;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
