// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-net-NoRouteToHostException"))]
__jni_bindgen! {
    /// public class [NoRouteToHostException](https://developer.android.com/reference/java/net/NoRouteToHostException.html)
    ///
    /// Required feature: java-net-NoRouteToHostException
    public class NoRouteToHostException ("java/net/NoRouteToHostException") extends crate::java::net::SocketException {

        /// [NoRouteToHostException](https://developer.android.com/reference/java/net/NoRouteToHostException.html#NoRouteToHostException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::NoRouteToHostException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/NoRouteToHostException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/NoRouteToHostException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [NoRouteToHostException](https://developer.android.com/reference/java/net/NoRouteToHostException.html#NoRouteToHostException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::net::NoRouteToHostException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/net/NoRouteToHostException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/net/NoRouteToHostException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
