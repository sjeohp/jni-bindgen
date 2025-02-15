// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-crypto-spec-DESKeySpec"))]
__jni_bindgen! {
    /// public class [DESKeySpec](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html)
    ///
    /// Required feature: javax-crypto-spec-DESKeySpec
    public class DESKeySpec ("javax/crypto/spec/DESKeySpec") extends crate::java::lang::Object, implements crate::java::security::spec::KeySpec {

        /// [DESKeySpec](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#DESKeySpec(byte%5B%5D))
        pub fn new_byte_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::crypto::spec::DESKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/DESKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "([B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/DESKeySpec\0", "<init>\0", "([B)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [DESKeySpec](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#DESKeySpec(byte%5B%5D,%20int))
        pub fn new_byte_array_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::crypto::spec::DESKeySpec>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/DESKeySpec", java.flags == PUBLIC, .name == "<init>", .descriptor == "([BI)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/DESKeySpec\0", "<init>\0", "([BI)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getKey](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#getKey())
        pub fn getKey<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/DESKeySpec", java.flags == PUBLIC, .name == "getKey", .descriptor == "()[B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/crypto/spec/DESKeySpec\0", "getKey\0", "()[B\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isParityAdjusted](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#isParityAdjusted(byte%5B%5D,%20int))
        pub fn isParityAdjusted<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/DESKeySpec", java.flags == PUBLIC | STATIC, .name == "isParityAdjusted", .descriptor == "([BI)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/crypto/spec/DESKeySpec\0", "isParityAdjusted\0", "([BI)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isWeak](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#isWeak(byte%5B%5D,%20int))
        pub fn isWeak<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/crypto/spec/DESKeySpec", java.flags == PUBLIC | STATIC, .name == "isWeak", .descriptor == "([BI)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("javax/crypto/spec/DESKeySpec\0", "isWeak\0", "([BI)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// public static final [DES_KEY_LEN](https://developer.android.com/reference/javax/crypto/spec/DESKeySpec.html#DES_KEY_LEN)
        pub const DES_KEY_LEN : i32 = 8;
    }
}
