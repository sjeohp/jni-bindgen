// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-SocketImplFactory"))]
__jni_bindgen! {
    /// public interface [SocketImplFactory](https://developer.android.com/reference/java/net/SocketImplFactory.html)
    ///
    /// Required feature: java-net-SocketImplFactory
    public interface SocketImplFactory ("java/net/SocketImplFactory") extends crate::java::lang::Object {

        /// [createSocketImpl](https://developer.android.com/reference/java/net/SocketImplFactory.html#createSocketImpl())
        ///
        /// Required features: "java-net-SocketImpl"
        #[cfg(any(feature = "all", all(feature = "java-net-SocketImpl")))]
        pub fn createSocketImpl<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::net::SocketImpl>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/SocketImplFactory", java.flags == PUBLIC | ABSTRACT, .name == "createSocketImpl", .descriptor == "()Ljava/net/SocketImpl;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/SocketImplFactory\0", "createSocketImpl\0", "()Ljava/net/SocketImpl;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
