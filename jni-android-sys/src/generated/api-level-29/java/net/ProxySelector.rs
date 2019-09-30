// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-ProxySelector"))]
__jni_bindgen! {
    /// public class [ProxySelector](https://developer.android.com/reference/java/net/ProxySelector.html)
    ///
    /// Required feature: java-net-ProxySelector
    public class ProxySelector ("java/net/ProxySelector") extends crate::java::lang::Object {

        /// [ProxySelector](https://developer.android.com/reference/java/net/ProxySelector.html#ProxySelector())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::ProxySelector>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/ProxySelector", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/ProxySelector\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDefault](https://developer.android.com/reference/java/net/ProxySelector.html#getDefault())
        ///
        /// Required features: "java-net-ProxySelector"
        #[cfg(any(feature = "all", all(feature = "java-net-ProxySelector")))]
        pub fn getDefault<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::ProxySelector>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/ProxySelector", java.flags == PUBLIC | STATIC, .name == "getDefault", .descriptor == "()Ljava/net/ProxySelector;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/ProxySelector\0", "getDefault\0", "()Ljava/net/ProxySelector;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDefault](https://developer.android.com/reference/java/net/ProxySelector.html#setDefault(java.net.ProxySelector))
        ///
        /// Required features: "java-net-ProxySelector"
        #[cfg(any(feature = "all", all(feature = "java-net-ProxySelector")))]
        pub fn setDefault<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::ProxySelector>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/ProxySelector", java.flags == PUBLIC | STATIC, .name == "setDefault", .descriptor == "(Ljava/net/ProxySelector;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/net/ProxySelector\0", "setDefault\0", "(Ljava/net/ProxySelector;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [select](https://developer.android.com/reference/java/net/ProxySelector.html#select(java.net.URI))
        ///
        /// Required features: "java-net-URI", "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-net-URI", feature = "java-util-List")))]
        pub fn select<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URI>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/ProxySelector", java.flags == PUBLIC | ABSTRACT, .name == "select", .descriptor == "(Ljava/net/URI;)Ljava/util/List;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/ProxySelector\0", "select\0", "(Ljava/net/URI;)Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connectFailed](https://developer.android.com/reference/java/net/ProxySelector.html#connectFailed(java.net.URI,%20java.net.SocketAddress,%20java.io.IOException))
        ///
        /// Required features: "java-io-IOException", "java-net-SocketAddress", "java-net-URI"
        #[cfg(any(feature = "all", all(feature = "java-io-IOException", feature = "java-net-SocketAddress", feature = "java-net-URI")))]
        pub fn connectFailed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::URI>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::IOException>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/ProxySelector", java.flags == PUBLIC | ABSTRACT, .name == "connectFailed", .descriptor == "(Ljava/net/URI;Ljava/net/SocketAddress;Ljava/io/IOException;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/ProxySelector\0", "connectFailed\0", "(Ljava/net/URI;Ljava/net/SocketAddress;Ljava/io/IOException;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
