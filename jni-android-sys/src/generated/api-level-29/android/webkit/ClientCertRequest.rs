// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-webkit-ClientCertRequest"))]
__jni_bindgen! {
    /// public class [ClientCertRequest](https://developer.android.com/reference/android/webkit/ClientCertRequest.html)
    ///
    /// Required feature: android-webkit-ClientCertRequest
    public class ClientCertRequest ("android/webkit/ClientCertRequest") extends crate::java::lang::Object {

        /// [ClientCertRequest](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#ClientCertRequest())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::webkit::ClientCertRequest>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyTypes](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#getKeyTypes())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getKeyTypes<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "getKeyTypes", .descriptor == "()[Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "getKeyTypes\0", "()[Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPrincipals](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#getPrincipals())
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn getPrincipals<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::security::Principal, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "getPrincipals", .descriptor == "()[Ljava/security/Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "getPrincipals\0", "()[Ljava/security/Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getHost](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#getHost())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getHost<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "getHost", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "getHost\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPort](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#getPort())
        pub fn getPort<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "getPort", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "getPort\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [proceed](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#proceed(java.security.PrivateKey,%20java.security.cert.X509Certificate%5B%5D))
        ///
        /// Required features: "java-security-PrivateKey", "java-security-cert-X509Certificate"
        #[cfg(any(feature = "all", all(feature = "java-security-PrivateKey", feature = "java-security-cert-X509Certificate")))]
        pub fn proceed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PrivateKey>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::security::cert::X509Certificate, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "proceed", .descriptor == "(Ljava/security/PrivateKey;[Ljava/security/cert/X509Certificate;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "proceed\0", "(Ljava/security/PrivateKey;[Ljava/security/cert/X509Certificate;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ignore](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#ignore())
        pub fn ignore<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "ignore", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "ignore\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [cancel](https://developer.android.com/reference/android/webkit/ClientCertRequest.html#cancel())
        pub fn cancel<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/webkit/ClientCertRequest", java.flags == PUBLIC | ABSTRACT, .name == "cancel", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/webkit/ClientCertRequest\0", "cancel\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
