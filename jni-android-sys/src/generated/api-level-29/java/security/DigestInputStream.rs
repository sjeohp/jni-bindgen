// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-security-DigestInputStream"))]
__jni_bindgen! {
    /// public class [DigestInputStream](https://developer.android.com/reference/java/security/DigestInputStream.html)
    ///
    /// Required feature: java-security-DigestInputStream
    public class DigestInputStream ("java/security/DigestInputStream") extends crate::java::io::FilterInputStream {

        /// [DigestInputStream](https://developer.android.com/reference/java/security/DigestInputStream.html#DigestInputStream(java.io.InputStream,%20java.security.MessageDigest))
        ///
        /// Required features: "java-io-InputStream", "java-security-MessageDigest"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-security-MessageDigest")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::MessageDigest>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::security::DigestInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;Ljava/security/MessageDigest;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "<init>\0", "(Ljava/io/InputStream;Ljava/security/MessageDigest;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMessageDigest](https://developer.android.com/reference/java/security/DigestInputStream.html#getMessageDigest())
        ///
        /// Required features: "java-security-MessageDigest"
        #[cfg(any(feature = "all", all(feature = "java-security-MessageDigest")))]
        pub fn getMessageDigest<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::MessageDigest>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "getMessageDigest", .descriptor == "()Ljava/security/MessageDigest;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "getMessageDigest\0", "()Ljava/security/MessageDigest;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setMessageDigest](https://developer.android.com/reference/java/security/DigestInputStream.html#setMessageDigest(java.security.MessageDigest))
        ///
        /// Required features: "java-security-MessageDigest"
        #[cfg(any(feature = "all", all(feature = "java-security-MessageDigest")))]
        pub fn setMessageDigest<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::security::MessageDigest>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "setMessageDigest", .descriptor == "(Ljava/security/MessageDigest;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "setMessageDigest\0", "(Ljava/security/MessageDigest;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/security/DigestInputStream.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/security/DigestInputStream.html#read(byte%5B%5D,%20int,%20int))
        pub fn read_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "read", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "read\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [on](https://developer.android.com/reference/java/security/DigestInputStream.html#on(boolean))
        pub fn on<'env>(&'env self, arg0: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "on", .descriptor == "(Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "on\0", "(Z)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/security/DigestInputStream.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/security/DigestInputStream", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/security/DigestInputStream\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected [digest](https://developer.android.com/reference/java/security/DigestInputStream.html#digest)
        // ///
        // /// Required feature: java-security-MessageDigest
        // #[cfg(any(feature = "all", feature = "java-security-MessageDigest"))]
        // pub fn digest<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::security::MessageDigest>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/security/DigestInputStream\0", "digest\0", "Ljava/security/MessageDigest;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [digest](https://developer.android.com/reference/java/security/DigestInputStream.html#digest)
        // ///
        // /// Required feature: java-security-MessageDigest
        // #[cfg(any(feature = "all", feature = "java-security-MessageDigest"))]
        // pub fn set_digest<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::security::MessageDigest>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/security/DigestInputStream\0", "digest\0", "Ljava/security/MessageDigest;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
