// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-MacSpi"))]
__jni_bindgen! {
    /// public class [MacSpi](https://developer.android.com/reference/javax/crypto/MacSpi.html)
    ///
    /// Required feature: javax-crypto-MacSpi
    public class MacSpi ("javax/crypto/MacSpi") extends crate::java::lang::Object {

        /// [MacSpi](https://developer.android.com/reference/javax/crypto/MacSpi.html#MacSpi())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::crypto::MacSpi>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/MacSpi", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [engineGetMacLength](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineGetMacLength())
        // fn engineGetMacLength<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineGetMacLength", .descriptor == "()I"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineGetMacLength\0", "()I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineInit](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineInit(java.security.Key,%20java.security.spec.AlgorithmParameterSpec))
        // ///
        // /// Required features: "java-security-Key", "java-security-spec-AlgorithmParameterSpec"
        // #[cfg(any(feature = "all", all(feature = "java-security-Key", feature = "java-security-spec-AlgorithmParameterSpec")))]
        // fn engineInit<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::Key>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::spec::AlgorithmParameterSpec>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineInit", .descriptor == "(Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineInit\0", "(Ljava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineUpdate](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineUpdate(byte))
        // fn engineUpdate<'env>(&'env self, arg0: i8) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineUpdate", .descriptor == "(B)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineUpdate\0", "(B)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineUpdate](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineUpdate(byte%5B%5D,%20int,%20int))
        // fn engineUpdate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineUpdate", .descriptor == "([BII)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineUpdate\0", "([BII)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineUpdate](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineUpdate(java.nio.ByteBuffer))
        // ///
        // /// Required features: "java-nio-ByteBuffer"
        // #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        // fn engineUpdate<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED, .name == "engineUpdate", .descriptor == "(Ljava/nio/ByteBuffer;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineUpdate\0", "(Ljava/nio/ByteBuffer;)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineDoFinal](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineDoFinal())
        // fn engineDoFinal<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineDoFinal", .descriptor == "()[B"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineDoFinal\0", "()[B\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Non-public method
        // /// [engineReset](https://developer.android.com/reference/javax/crypto/MacSpi.html#engineReset())
        // fn engineReset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "javax/crypto/MacSpi", java.flags == PROTECTED | ABSTRACT, .name == "engineReset", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "engineReset\0", "()V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [clone](https://developer.android.com/reference/javax/crypto/MacSpi.html#clone())
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn clone<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/MacSpi", java.flags == PUBLIC, .name == "clone", .descriptor == "()Ljava/lang/Object;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/MacSpi\0", "clone\0", "()Ljava/lang/Object;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
