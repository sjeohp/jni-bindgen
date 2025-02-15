// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-AlgorithmParameterGeneratorSpi"))]
__jni_bindgen! {
    /// public class [AlgorithmParameterGeneratorSpi](https://developer.android.com/reference/java/security/AlgorithmParameterGeneratorSpi.html)
    ///
    /// Required feature: java-security-AlgorithmParameterGeneratorSpi
    public class AlgorithmParameterGeneratorSpi ("java/security/AlgorithmParameterGeneratorSpi") extends crate::java::lang::Object {

        /// [AlgorithmParameterGeneratorSpi](https://developer.android.com/reference/java/security/AlgorithmParameterGeneratorSpi.html#AlgorithmParameterGeneratorSpi())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::AlgorithmParameterGeneratorSpi>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/AlgorithmParameterGeneratorSpi", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmParameterGeneratorSpi\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [engineInit](https://developer.android.com/reference/java/security/AlgorithmParameterGeneratorSpi.html#engineInit(int,%20java.security.SecureRandom))
        // ///
        // /// Required features: "java-security-SecureRandom"
        // #[cfg(any(feature = "all", all(feature = "java-security-SecureRandom")))]
        // fn engineInit<'env>(&'env self, arg0: i32, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::SecureRandom>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/AlgorithmParameterGeneratorSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineInit", .descriptor == "(ILjava/security/SecureRandom;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmParameterGeneratorSpi\0", "engineInit\0", "(ILjava/security/SecureRandom;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineInit](https://developer.android.com/reference/java/security/AlgorithmParameterGeneratorSpi.html#engineInit(java.security.spec.AlgorithmParameterSpec,%20java.security.SecureRandom))
        // ///
        // /// Required features: "java-security-SecureRandom", "java-security-spec-AlgorithmParameterSpec"
        // #[cfg(any(feature = "all", all(feature = "java-security-SecureRandom", feature = "java-security-spec-AlgorithmParameterSpec")))]
        // fn engineInit<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::SecureRandom>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/AlgorithmParameterGeneratorSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineInit", .descriptor == "(Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmParameterGeneratorSpi\0", "engineInit\0", "(Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineGenerateParameters](https://developer.android.com/reference/java/security/AlgorithmParameterGeneratorSpi.html#engineGenerateParameters())
        // ///
        // /// Required features: "java-security-AlgorithmParameters"
        // #[cfg(any(feature = "all", all(feature = "java-security-AlgorithmParameters")))]
        // fn engineGenerateParameters<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::AlgorithmParameters>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/security/AlgorithmParameterGeneratorSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineGenerateParameters", .descriptor == "()Ljava/security/AlgorithmParameters;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/AlgorithmParameterGeneratorSpi\0", "engineGenerateParameters\0", "()Ljava/security/AlgorithmParameters;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
