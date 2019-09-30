// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-Timestamp"))]
__jni_bindgen! {
    /// public final class [Timestamp](https://developer.android.com/reference/java/security/Timestamp.html)
    ///
    /// Required feature: java-security-Timestamp
    public final class Timestamp ("java/security/Timestamp") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        /// [Timestamp](https://developer.android.com/reference/java/security/Timestamp.html#Timestamp(java.util.Date,%20java.security.cert.CertPath))
        ///
        /// Required features: "java-security-cert-CertPath", "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPath", feature = "java-util-Date")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertPath>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::Timestamp>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Date;Ljava/security/cert/CertPath;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "<init>\0", "(Ljava/util/Date;Ljava/security/cert/CertPath;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTimestamp](https://developer.android.com/reference/java/security/Timestamp.html#getTimestamp())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getTimestamp<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "getTimestamp", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "getTimestamp\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSignerCertPath](https://developer.android.com/reference/java/security/Timestamp.html#getSignerCertPath())
        ///
        /// Required features: "java-security-cert-CertPath"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertPath")))]
        pub fn getSignerCertPath<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertPath>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "getSignerCertPath", .descriptor == "()Ljava/security/cert/CertPath;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "getSignerCertPath\0", "()Ljava/security/cert/CertPath;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/security/Timestamp.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/security/Timestamp.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/Timestamp.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/Timestamp", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/Timestamp\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
