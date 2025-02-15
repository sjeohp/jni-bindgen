// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-X509Certificate"))]
__jni_bindgen! {
    /// public class [X509Certificate](https://developer.android.com/reference/java/security/cert/X509Certificate.html)
    ///
    /// Required feature: java-security-cert-X509Certificate
    public class X509Certificate ("java/security/cert/X509Certificate") extends crate::java::security::cert::Certificate, implements crate::java::security::cert::X509Extension {

        // // Not emitting: Non-public method
        // /// [X509Certificate](https://developer.android.com/reference/java/security/cert/X509Certificate.html#X509Certificate())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::X509Certificate>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/cert/X509Certificate", java.flags == PROTECTED, .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [checkValidity](https://developer.android.com/reference/java/security/cert/X509Certificate.html#checkValidity())
        pub fn checkValidity<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "checkValidity", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "checkValidity\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [checkValidity](https://developer.android.com/reference/java/security/cert/X509Certificate.html#checkValidity(java.util.Date))
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn checkValidity_Date<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "checkValidity", .descriptor == "(Ljava/util/Date;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "checkValidity\0", "(Ljava/util/Date;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getVersion](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getVersion())
        pub fn getVersion<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getVersion", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getVersion\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSerialNumber](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSerialNumber())
        ///
        /// Required features: "java-math-BigInteger"
        #[cfg(any(feature = "all", all(feature = "java-math-BigInteger")))]
        pub fn getSerialNumber<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::math::BigInteger>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSerialNumber", .descriptor == "()Ljava/math/BigInteger;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSerialNumber\0", "()Ljava/math/BigInteger;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIssuerDN](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getIssuerDN())
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn getIssuerDN<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getIssuerDN", .descriptor == "()Ljava/security/Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getIssuerDN\0", "()Ljava/security/Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIssuerX500Principal](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getIssuerX500Principal())
        ///
        /// Required features: "javax-security-auth-x500-X500Principal"
        #[cfg(any(feature = "all", all(feature = "javax-security-auth-x500-X500Principal")))]
        pub fn getIssuerX500Principal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::security::auth::x500::X500Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "getIssuerX500Principal", .descriptor == "()Ljavax/security/auth/x500/X500Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getIssuerX500Principal\0", "()Ljavax/security/auth/x500/X500Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubjectDN](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSubjectDN())
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn getSubjectDN<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSubjectDN", .descriptor == "()Ljava/security/Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSubjectDN\0", "()Ljava/security/Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubjectX500Principal](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSubjectX500Principal())
        ///
        /// Required features: "javax-security-auth-x500-X500Principal"
        #[cfg(any(feature = "all", all(feature = "javax-security-auth-x500-X500Principal")))]
        pub fn getSubjectX500Principal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::javax::security::auth::x500::X500Principal>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "getSubjectX500Principal", .descriptor == "()Ljavax/security/auth/x500/X500Principal;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSubjectX500Principal\0", "()Ljavax/security/auth/x500/X500Principal;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNotBefore](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getNotBefore())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getNotBefore<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getNotBefore", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getNotBefore\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getNotAfter](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getNotAfter())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getNotAfter<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getNotAfter", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getNotAfter\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTBSCertificate](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getTBSCertificate())
        pub fn getTBSCertificate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getTBSCertificate", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getTBSCertificate\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSignature](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSignature())
        pub fn getSignature<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSignature", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSignature\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSigAlgName](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSigAlgName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSigAlgName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSigAlgName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSigAlgName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSigAlgOID](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSigAlgOID())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSigAlgOID<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSigAlgOID", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSigAlgOID\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSigAlgParams](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSigAlgParams())
        pub fn getSigAlgParams<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSigAlgParams", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSigAlgParams\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIssuerUniqueID](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getIssuerUniqueID())
        pub fn getIssuerUniqueID<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::BooleanArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getIssuerUniqueID", .descriptor == "()[Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getIssuerUniqueID\0", "()[Z\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubjectUniqueID](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSubjectUniqueID())
        pub fn getSubjectUniqueID<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::BooleanArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getSubjectUniqueID", .descriptor == "()[Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSubjectUniqueID\0", "()[Z\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKeyUsage](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getKeyUsage())
        pub fn getKeyUsage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::BooleanArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getKeyUsage", .descriptor == "()[Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getKeyUsage\0", "()[Z\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getExtendedKeyUsage](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getExtendedKeyUsage())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getExtendedKeyUsage<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "getExtendedKeyUsage", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getExtendedKeyUsage\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBasicConstraints](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getBasicConstraints())
        pub fn getBasicConstraints<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC | ABSTRACT, .name == "getBasicConstraints", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getBasicConstraints\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubjectAlternativeNames](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getSubjectAlternativeNames())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn getSubjectAlternativeNames<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "getSubjectAlternativeNames", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getSubjectAlternativeNames\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIssuerAlternativeNames](https://developer.android.com/reference/java/security/cert/X509Certificate.html#getIssuerAlternativeNames())
        ///
        /// Required features: "java-util-Collection"
        #[cfg(any(feature = "all", all(feature = "java-util-Collection")))]
        pub fn getIssuerAlternativeNames<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Collection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "getIssuerAlternativeNames", .descriptor == "()Ljava/util/Collection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "getIssuerAlternativeNames\0", "()Ljava/util/Collection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [verify](https://developer.android.com/reference/java/security/cert/X509Certificate.html#verify(java.security.PublicKey,%20java.security.Provider))
        ///
        /// Required features: "java-security-Provider", "java-security-PublicKey"
        #[cfg(any(feature = "all", all(feature = "java-security-Provider", feature = "java-security-PublicKey")))]
        pub fn verify<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::PublicKey>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Provider>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/X509Certificate", java.flags == PUBLIC, .name == "verify", .descriptor == "(Ljava/security/PublicKey;Ljava/security/Provider;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/X509Certificate\0", "verify\0", "(Ljava/security/PublicKey;Ljava/security/Provider;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
