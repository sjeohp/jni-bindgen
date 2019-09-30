// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-SecureRandomSpi"))]
__jni_bindgen! {
    /// public class [SecureRandomSpi](https://developer.android.com/reference/java/security/SecureRandomSpi.html)
    ///
    /// Required feature: java-security-SecureRandomSpi
    public class SecureRandomSpi ("java/security/SecureRandomSpi") extends crate::java::lang::Object, implements crate::java::io::Serializable {

        /// [SecureRandomSpi](https://developer.android.com/reference/java/security/SecureRandomSpi.html#SecureRandomSpi())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::SecureRandomSpi>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/SecureRandomSpi", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/SecureRandomSpi\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [engineSetSeed](https://developer.android.com/reference/java/security/SecureRandomSpi.html#engineSetSeed(byte%5B%5D))
        // fn engineSetSeed<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/SecureRandomSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineSetSeed", .descriptor == "([B)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/SecureRandomSpi\0", "engineSetSeed\0", "([B)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineNextBytes](https://developer.android.com/reference/java/security/SecureRandomSpi.html#engineNextBytes(byte%5B%5D))
        // fn engineNextBytes<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/SecureRandomSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineNextBytes", .descriptor == "([B)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/SecureRandomSpi\0", "engineNextBytes\0", "([B)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineGenerateSeed](https://developer.android.com/reference/java/security/SecureRandomSpi.html#engineGenerateSeed(int))
        // fn engineGenerateSeed<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/SecureRandomSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineGenerateSeed", .descriptor == "(I)[B"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/SecureRandomSpi\0", "engineGenerateSeed\0", "(I)[B\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
