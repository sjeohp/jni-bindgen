// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-KeyStoreBuilderParameters"))]
__jni_bindgen! {
    /// public class [KeyStoreBuilderParameters](https://developer.android.com/reference/javax/net/ssl/KeyStoreBuilderParameters.html)
    ///
    /// Required feature: javax-net-ssl-KeyStoreBuilderParameters
    public class KeyStoreBuilderParameters ("javax/net/ssl/KeyStoreBuilderParameters") extends crate::java::lang::Object, implements crate::javax::net::ssl::ManagerFactoryParameters {

        /// [KeyStoreBuilderParameters](https://developer.android.com/reference/javax/net/ssl/KeyStoreBuilderParameters.html#KeyStoreBuilderParameters(java.security.KeyStore.Builder))
        ///
        /// Required features: "java-security-KeyStore_Builder"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyStore_Builder")))]
        pub fn new_Builder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyStore_Builder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::KeyStoreBuilderParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/KeyStoreBuilderParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/KeyStore$Builder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyStoreBuilderParameters\0", "<init>\0", "(Ljava/security/KeyStore$Builder;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [KeyStoreBuilderParameters](https://developer.android.com/reference/javax/net/ssl/KeyStoreBuilderParameters.html#KeyStoreBuilderParameters(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn new_List<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::KeyStoreBuilderParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/KeyStoreBuilderParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyStoreBuilderParameters\0", "<init>\0", "(Ljava/util/List;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getParameters](https://developer.android.com/reference/javax/net/ssl/KeyStoreBuilderParameters.html#getParameters())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getParameters<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/KeyStoreBuilderParameters", java.flags == PUBLIC, .name == "getParameters", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/KeyStoreBuilderParameters\0", "getParameters\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
