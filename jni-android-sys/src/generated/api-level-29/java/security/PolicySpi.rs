// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-PolicySpi"))]
__jni_bindgen! {
    /// public class [PolicySpi](https://developer.android.com/reference/java/security/PolicySpi.html)
    ///
    /// Required feature: java-security-PolicySpi
    public class PolicySpi ("java/security/PolicySpi") extends crate::java::lang::Object {

        /// [PolicySpi](https://developer.android.com/reference/java/security/PolicySpi.html#PolicySpi())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::PolicySpi>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/PolicySpi", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/PolicySpi\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [engineImplies](https://developer.android.com/reference/java/security/PolicySpi.html#engineImplies(java.security.ProtectionDomain,%20java.security.Permission))
        // ///
        // /// Required features: "java-security-Permission", "java-security-ProtectionDomain"
        // #[cfg(any(feature = "all", all(feature = "java-security-Permission", feature = "java-security-ProtectionDomain")))]
        // fn engineImplies<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::ProtectionDomain>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Permission>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/PolicySpi", java.flags == PROTECTED | ABSTRACT, .name == "engineImplies", .descriptor == "(Ljava/security/ProtectionDomain;Ljava/security/Permission;)Z"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/PolicySpi\0", "engineImplies\0", "(Ljava/security/ProtectionDomain;Ljava/security/Permission;)Z\0");
        //         __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineRefresh](https://developer.android.com/reference/java/security/PolicySpi.html#engineRefresh())
        // fn engineRefresh<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/PolicySpi", java.flags == PROTECTED, .name == "engineRefresh", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/PolicySpi\0", "engineRefresh\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineGetPermissions](https://developer.android.com/reference/java/security/PolicySpi.html#engineGetPermissions(java.security.CodeSource))
        // ///
        // /// Required features: "java-security-CodeSource", "java-security-PermissionCollection"
        // #[cfg(any(feature = "all", all(feature = "java-security-CodeSource", feature = "java-security-PermissionCollection")))]
        // fn engineGetPermissions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::CodeSource>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::PermissionCollection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/PolicySpi", java.flags == PROTECTED, .name == "engineGetPermissions", .descriptor == "(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/PolicySpi\0", "engineGetPermissions\0", "(Ljava/security/CodeSource;)Ljava/security/PermissionCollection;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineGetPermissions](https://developer.android.com/reference/java/security/PolicySpi.html#engineGetPermissions(java.security.ProtectionDomain))
        // ///
        // /// Required features: "java-security-PermissionCollection", "java-security-ProtectionDomain"
        // #[cfg(any(feature = "all", all(feature = "java-security-PermissionCollection", feature = "java-security-ProtectionDomain")))]
        // fn engineGetPermissions<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::ProtectionDomain>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::PermissionCollection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/PolicySpi", java.flags == PROTECTED, .name == "engineGetPermissions", .descriptor == "(Ljava/security/ProtectionDomain;)Ljava/security/PermissionCollection;"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/PolicySpi\0", "engineGetPermissions\0", "(Ljava/security/ProtectionDomain;)Ljava/security/PermissionCollection;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
