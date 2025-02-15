// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-interfaces-PBEKey"))]
__jni_bindgen! {
    /// public interface [PBEKey](https://developer.android.com/reference/javax/crypto/interfaces/PBEKey.html)
    ///
    /// Required feature: javax-crypto-interfaces-PBEKey
    public interface PBEKey ("javax/crypto/interfaces/PBEKey") extends crate::java::lang::Object, implements crate::javax::crypto::SecretKey {

        /// [getPassword](https://developer.android.com/reference/javax/crypto/interfaces/PBEKey.html#getPassword())
        pub fn getPassword<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/interfaces/PBEKey", java.flags == PUBLIC | ABSTRACT, .name == "getPassword", .descriptor == "()[C"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/interfaces/PBEKey\0", "getPassword\0", "()[C\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSalt](https://developer.android.com/reference/javax/crypto/interfaces/PBEKey.html#getSalt())
        pub fn getSalt<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/interfaces/PBEKey", java.flags == PUBLIC | ABSTRACT, .name == "getSalt", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/interfaces/PBEKey\0", "getSalt\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getIterationCount](https://developer.android.com/reference/javax/crypto/interfaces/PBEKey.html#getIterationCount())
        pub fn getIterationCount<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/interfaces/PBEKey", java.flags == PUBLIC | ABSTRACT, .name == "getIterationCount", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/interfaces/PBEKey\0", "getIterationCount\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [serialVersionUID](https://developer.android.com/reference/javax/crypto/interfaces/PBEKey.html#serialVersionUID)
        pub const serialVersionUID : i64 = -1430015993304333921i64;
    }
}
