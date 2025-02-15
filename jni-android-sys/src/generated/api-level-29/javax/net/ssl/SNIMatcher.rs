// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-SNIMatcher"))]
__jni_bindgen! {
    /// public class [SNIMatcher](https://developer.android.com/reference/javax/net/ssl/SNIMatcher.html)
    ///
    /// Required feature: javax-net-ssl-SNIMatcher
    public class SNIMatcher ("javax/net/ssl/SNIMatcher") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [SNIMatcher](https://developer.android.com/reference/javax/net/ssl/SNIMatcher.html#SNIMatcher(int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SNIMatcher>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/net/ssl/SNIMatcher", java.flags == PROTECTED, .name == "<init>", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SNIMatcher\0", "<init>\0", "(I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [getType](https://developer.android.com/reference/javax/net/ssl/SNIMatcher.html#getType())
        pub fn getType<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SNIMatcher", java.flags == PUBLIC | FINAL, .name == "getType", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SNIMatcher\0", "getType\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [matches](https://developer.android.com/reference/javax/net/ssl/SNIMatcher.html#matches(javax.net.ssl.SNIServerName))
        ///
        /// Required features: "javax-net-ssl-SNIServerName"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SNIServerName")))]
        pub fn matches<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SNIServerName>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SNIMatcher", java.flags == PUBLIC | ABSTRACT, .name == "matches", .descriptor == "(Ljavax/net/ssl/SNIServerName;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SNIMatcher\0", "matches\0", "(Ljavax/net/ssl/SNIServerName;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
