// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-cert-PKIXParameters"))]
__jni_bindgen! {
    /// public class [PKIXParameters](https://developer.android.com/reference/java/security/cert/PKIXParameters.html)
    ///
    /// Required feature: java-security-cert-PKIXParameters
    public class PKIXParameters ("java/security/cert/PKIXParameters") extends crate::java::lang::Object, implements crate::java::security::cert::CertPathParameters {

        /// [PKIXParameters](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#PKIXParameters(java.util.Set))
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn new_Set<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/util/Set;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "<init>\0", "(Ljava/util/Set;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PKIXParameters](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#PKIXParameters(java.security.KeyStore))
        ///
        /// Required features: "java-security-KeyStore"
        #[cfg(any(feature = "all", all(feature = "java-security-KeyStore")))]
        pub fn new_KeyStore<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::KeyStore>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::cert::PKIXParameters>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/security/KeyStore;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "<init>\0", "(Ljava/security/KeyStore;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTrustAnchors](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getTrustAnchors())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getTrustAnchors<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getTrustAnchors", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getTrustAnchors\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTrustAnchors](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setTrustAnchors(java.util.Set))
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn setTrustAnchors<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setTrustAnchors", .descriptor == "(Ljava/util/Set;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setTrustAnchors\0", "(Ljava/util/Set;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInitialPolicies](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getInitialPolicies())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn getInitialPolicies<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getInitialPolicies", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getInitialPolicies\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInitialPolicies](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setInitialPolicies(java.util.Set))
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn setInitialPolicies<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Set>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setInitialPolicies", .descriptor == "(Ljava/util/Set;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setInitialPolicies\0", "(Ljava/util/Set;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCertStores](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setCertStores(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn setCertStores<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setCertStores", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setCertStores\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCertStore](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#addCertStore(java.security.cert.CertStore))
        ///
        /// Required features: "java-security-cert-CertStore"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertStore")))]
        pub fn addCertStore<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertStore>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "addCertStore", .descriptor == "(Ljava/security/cert/CertStore;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "addCertStore\0", "(Ljava/security/cert/CertStore;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertStores](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getCertStores())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getCertStores<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getCertStores", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getCertStores\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setRevocationEnabled](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setRevocationEnabled(boolean))
        pub fn setRevocationEnabled<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setRevocationEnabled", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setRevocationEnabled\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRevocationEnabled](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#isRevocationEnabled())
        pub fn isRevocationEnabled<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "isRevocationEnabled", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "isRevocationEnabled\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setExplicitPolicyRequired](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setExplicitPolicyRequired(boolean))
        pub fn setExplicitPolicyRequired<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setExplicitPolicyRequired", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setExplicitPolicyRequired\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isExplicitPolicyRequired](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#isExplicitPolicyRequired())
        pub fn isExplicitPolicyRequired<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "isExplicitPolicyRequired", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "isExplicitPolicyRequired\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPolicyMappingInhibited](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setPolicyMappingInhibited(boolean))
        pub fn setPolicyMappingInhibited<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setPolicyMappingInhibited", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setPolicyMappingInhibited\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isPolicyMappingInhibited](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#isPolicyMappingInhibited())
        pub fn isPolicyMappingInhibited<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "isPolicyMappingInhibited", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "isPolicyMappingInhibited\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAnyPolicyInhibited](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setAnyPolicyInhibited(boolean))
        pub fn setAnyPolicyInhibited<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setAnyPolicyInhibited", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setAnyPolicyInhibited\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isAnyPolicyInhibited](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#isAnyPolicyInhibited())
        pub fn isAnyPolicyInhibited<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "isAnyPolicyInhibited", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "isAnyPolicyInhibited\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setPolicyQualifiersRejected](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setPolicyQualifiersRejected(boolean))
        pub fn setPolicyQualifiersRejected<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setPolicyQualifiersRejected", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setPolicyQualifiersRejected\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPolicyQualifiersRejected](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getPolicyQualifiersRejected())
        pub fn getPolicyQualifiersRejected<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getPolicyQualifiersRejected", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getPolicyQualifiersRejected\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDate](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getDate())
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn getDate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Date>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getDate", .descriptor == "()Ljava/util/Date;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getDate\0", "()Ljava/util/Date;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDate](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setDate(java.util.Date))
        ///
        /// Required features: "java-util-Date"
        #[cfg(any(feature = "all", all(feature = "java-util-Date")))]
        pub fn setDate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Date>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setDate", .descriptor == "(Ljava/util/Date;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setDate\0", "(Ljava/util/Date;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCertPathCheckers](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setCertPathCheckers(java.util.List))
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn setCertPathCheckers<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::List>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setCertPathCheckers", .descriptor == "(Ljava/util/List;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setCertPathCheckers\0", "(Ljava/util/List;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCertPathCheckers](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getCertPathCheckers())
        ///
        /// Required features: "java-util-List"
        #[cfg(any(feature = "all", all(feature = "java-util-List")))]
        pub fn getCertPathCheckers<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::List>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getCertPathCheckers", .descriptor == "()Ljava/util/List;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getCertPathCheckers\0", "()Ljava/util/List;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addCertPathChecker](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#addCertPathChecker(java.security.cert.PKIXCertPathChecker))
        ///
        /// Required features: "java-security-cert-PKIXCertPathChecker"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-PKIXCertPathChecker")))]
        pub fn addCertPathChecker<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::PKIXCertPathChecker>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "addCertPathChecker", .descriptor == "(Ljava/security/cert/PKIXCertPathChecker;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "addCertPathChecker\0", "(Ljava/security/cert/PKIXCertPathChecker;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSigProvider](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getSigProvider())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSigProvider<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getSigProvider", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getSigProvider\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setSigProvider](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setSigProvider(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setSigProvider<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setSigProvider", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setSigProvider\0", "(Ljava/lang/String;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getTargetCertConstraints](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#getTargetCertConstraints())
        ///
        /// Required features: "java-security-cert-CertSelector"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertSelector")))]
        pub fn getTargetCertConstraints<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::cert::CertSelector>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "getTargetCertConstraints", .descriptor == "()Ljava/security/cert/CertSelector;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "getTargetCertConstraints\0", "()Ljava/security/cert/CertSelector;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setTargetCertConstraints](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#setTargetCertConstraints(java.security.cert.CertSelector))
        ///
        /// Required features: "java-security-cert-CertSelector"
        #[cfg(any(feature = "all", all(feature = "java-security-cert-CertSelector")))]
        pub fn setTargetCertConstraints<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::cert::CertSelector>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "setTargetCertConstraints", .descriptor == "(Ljava/security/cert/CertSelector;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "setTargetCertConstraints\0", "(Ljava/security/cert/CertSelector;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clone](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/cert/PKIXParameters.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/cert/PKIXParameters", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/cert/PKIXParameters\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
