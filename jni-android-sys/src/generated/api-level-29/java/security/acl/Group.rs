// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-acl-Group"))]
__jni_bindgen! {
    /// public interface [Group](https://developer.android.com/reference/java/security/acl/Group.html)
    ///
    /// Required feature: java-security-acl-Group
    public interface Group ("java/security/acl/Group") extends crate::java::lang::Object, implements crate::java::security::Principal {

        /// [addMember](https://developer.android.com/reference/java/security/acl/Group.html#addMember(java.security.Principal))
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn addMember<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Principal>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/acl/Group", java.flags == PUBLIC | ABSTRACT, .name == "addMember", .descriptor == "(Ljava/security/Principal;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/acl/Group\0", "addMember\0", "(Ljava/security/Principal;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeMember](https://developer.android.com/reference/java/security/acl/Group.html#removeMember(java.security.Principal))
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn removeMember<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Principal>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/acl/Group", java.flags == PUBLIC | ABSTRACT, .name == "removeMember", .descriptor == "(Ljava/security/Principal;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/acl/Group\0", "removeMember\0", "(Ljava/security/Principal;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isMember](https://developer.android.com/reference/java/security/acl/Group.html#isMember(java.security.Principal))
        ///
        /// Required features: "java-security-Principal"
        #[cfg(any(feature = "all", all(feature = "java-security-Principal")))]
        pub fn isMember<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Principal>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/acl/Group", java.flags == PUBLIC | ABSTRACT, .name == "isMember", .descriptor == "(Ljava/security/Principal;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/acl/Group\0", "isMember\0", "(Ljava/security/Principal;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [members](https://developer.android.com/reference/java/security/acl/Group.html#members())
        ///
        /// Required features: "java-util-Enumeration"
        #[cfg(any(feature = "all", all(feature = "java-util-Enumeration")))]
        pub fn members<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Enumeration>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/acl/Group", java.flags == PUBLIC | ABSTRACT, .name == "members", .descriptor == "()Ljava/util/Enumeration;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/acl/Group\0", "members\0", "()Ljava/util/Enumeration;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
