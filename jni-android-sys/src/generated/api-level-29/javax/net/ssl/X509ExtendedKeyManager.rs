// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-X509ExtendedKeyManager"))]
__jni_bindgen! {
    /// public class [X509ExtendedKeyManager](https://developer.android.com/reference/javax/net/ssl/X509ExtendedKeyManager.html)
    ///
    /// Required feature: javax-net-ssl-X509ExtendedKeyManager
    public class X509ExtendedKeyManager ("javax/net/ssl/X509ExtendedKeyManager") extends crate::java::lang::Object, implements crate::javax::net::ssl::X509KeyManager {

        // // Not emitting: Non-public method
        // /// [X509ExtendedKeyManager](https://developer.android.com/reference/javax/net/ssl/X509ExtendedKeyManager.html#X509ExtendedKeyManager())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::X509ExtendedKeyManager>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/X509ExtendedKeyManager", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/X509ExtendedKeyManager\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [chooseEngineClientAlias](https://developer.android.com/reference/javax/net/ssl/X509ExtendedKeyManager.html#chooseEngineClientAlias(java.lang.String%5B%5D,%20java.security.Principal%5B%5D,%20javax.net.ssl.SSLEngine))
        ///
        /// Required features: "java-lang-String", "java-security-Principal", "javax-net-ssl-SSLEngine"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Principal", feature = "javax-net-ssl-SSLEngine")))]
        pub fn chooseEngineClientAlias<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::Principal, crate::java::lang::Throwable>>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLEngine>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/X509ExtendedKeyManager", java.flags == PUBLIC, .name == "chooseEngineClientAlias", .descriptor == "([Ljava/lang/String;[Ljava/security/Principal;Ljavax/net/ssl/SSLEngine;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/X509ExtendedKeyManager\0", "chooseEngineClientAlias\0", "([Ljava/lang/String;[Ljava/security/Principal;Ljavax/net/ssl/SSLEngine;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [chooseEngineServerAlias](https://developer.android.com/reference/javax/net/ssl/X509ExtendedKeyManager.html#chooseEngineServerAlias(java.lang.String,%20java.security.Principal%5B%5D,%20javax.net.ssl.SSLEngine))
        ///
        /// Required features: "java-lang-String", "java-security-Principal", "javax-net-ssl-SSLEngine"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-security-Principal", feature = "javax-net-ssl-SSLEngine")))]
        pub fn chooseEngineServerAlias<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::Principal, crate::java::lang::Throwable>>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLEngine>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/X509ExtendedKeyManager", java.flags == PUBLIC, .name == "chooseEngineServerAlias", .descriptor == "(Ljava/lang/String;[Ljava/security/Principal;Ljavax/net/ssl/SSLEngine;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/X509ExtendedKeyManager\0", "chooseEngineServerAlias\0", "(Ljava/lang/String;[Ljava/security/Principal;Ljavax/net/ssl/SSLEngine;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
