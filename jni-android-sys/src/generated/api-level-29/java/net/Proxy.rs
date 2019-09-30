// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-Proxy"))]
__jni_bindgen! {
    /// public class [Proxy](https://developer.android.com/reference/java/net/Proxy.html)
    ///
    /// Required feature: java-net-Proxy
    public class Proxy ("java/net/Proxy") extends crate::java::lang::Object {

        /// [Proxy](https://developer.android.com/reference/java/net/Proxy.html#Proxy(java.net.Proxy.Type,%20java.net.SocketAddress))
        ///
        /// Required features: "java-net-Proxy_Type", "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-Proxy_Type", feature = "java-net-SocketAddress")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::Proxy_Type>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::net::SocketAddress>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::Proxy>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/net/Proxy$Type;Ljava/net/SocketAddress;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "<init>\0", "(Ljava/net/Proxy$Type;Ljava/net/SocketAddress;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [type](https://developer.android.com/reference/java/net/Proxy.html#type())
        ///
        /// Required features: "java-net-Proxy_Type"
        #[cfg(any(feature = "all", all(feature = "java-net-Proxy_Type")))]
        pub fn r#type<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy_Type>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC, .name == "type", .descriptor == "()Ljava/net/Proxy$Type;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "type\0", "()Ljava/net/Proxy$Type;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [address](https://developer.android.com/reference/java/net/Proxy.html#address())
        ///
        /// Required features: "java-net-SocketAddress"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketAddress")))]
        pub fn address<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::SocketAddress>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC, .name == "address", .descriptor == "()Ljava/net/SocketAddress;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "address\0", "()Ljava/net/SocketAddress;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/net/Proxy.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/net/Proxy.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC | FINAL, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/net/Proxy.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/Proxy", java.flags == PUBLIC | FINAL, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/Proxy\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// **get** public static final [NO_PROXY](https://developer.android.com/reference/java/net/Proxy.html#NO_PROXY)
        ///
        /// Required feature: java-net-Proxy
        #[cfg(any(feature = "all", feature = "java-net-Proxy"))]
        pub fn NO_PROXY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::Proxy>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/net/Proxy\0", "NO_PROXY\0", "Ljava/net/Proxy;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
