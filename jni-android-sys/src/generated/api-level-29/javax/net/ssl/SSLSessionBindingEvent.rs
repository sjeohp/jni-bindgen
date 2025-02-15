// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-net-ssl-SSLSessionBindingEvent"))]
__jni_bindgen! {
    /// public class [SSLSessionBindingEvent](https://developer.android.com/reference/javax/net/ssl/SSLSessionBindingEvent.html)
    ///
    /// Required feature: javax-net-ssl-SSLSessionBindingEvent
    public class SSLSessionBindingEvent ("javax/net/ssl/SSLSessionBindingEvent") extends crate::java::util::EventObject {

        /// [SSLSessionBindingEvent](https://developer.android.com/reference/javax/net/ssl/SSLSessionBindingEvent.html#SSLSessionBindingEvent(javax.net.ssl.SSLSession,%20java.lang.String))
        ///
        /// Required features: "java-lang-String", "javax-net-ssl-SSLSession"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "javax-net-ssl-SSLSession")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::net::ssl::SSLSession>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSessionBindingEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSessionBindingEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/net/ssl/SSLSession;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSessionBindingEvent\0", "<init>\0", "(Ljavax/net/ssl/SSLSession;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getName](https://developer.android.com/reference/javax/net/ssl/SSLSessionBindingEvent.html#getName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSessionBindingEvent", java.flags == PUBLIC, .name == "getName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSessionBindingEvent\0", "getName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSession](https://developer.android.com/reference/javax/net/ssl/SSLSessionBindingEvent.html#getSession())
        ///
        /// Required features: "javax-net-ssl-SSLSession"
        #[cfg(any(feature = "all", all(feature = "javax-net-ssl-SSLSession")))]
        pub fn getSession<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::net::ssl::SSLSession>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/net/ssl/SSLSessionBindingEvent", java.flags == PUBLIC, .name == "getSession", .descriptor == "()Ljavax/net/ssl/SSLSession;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/net/ssl/SSLSessionBindingEvent\0", "getSession\0", "()Ljavax/net/ssl/SSLSession;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
