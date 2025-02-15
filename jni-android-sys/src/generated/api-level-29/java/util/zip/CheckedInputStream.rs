// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-zip-CheckedInputStream"))]
__jni_bindgen! {
    /// public class [CheckedInputStream](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html)
    ///
    /// Required feature: java-util-zip-CheckedInputStream
    public class CheckedInputStream ("java/util/zip/CheckedInputStream") extends crate::java::io::FilterInputStream {

        /// [CheckedInputStream](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html#CheckedInputStream(java.io.InputStream,%20java.util.zip.Checksum))
        ///
        /// Required features: "java-io-InputStream", "java-util-zip-Checksum"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-util-zip-Checksum")))]
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::zip::Checksum>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::CheckedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/CheckedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;Ljava/util/zip/Checksum;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/CheckedInputStream\0", "<init>\0", "(Ljava/io/InputStream;Ljava/util/zip/Checksum;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/CheckedInputStream", java.flags == PUBLIC, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/CheckedInputStream\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html#read(byte%5B%5D,%20int,%20int))
        pub fn read_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/CheckedInputStream", java.flags == PUBLIC, .name == "read", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/CheckedInputStream\0", "read\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [skip](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html#skip(long))
        pub fn skip<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/CheckedInputStream", java.flags == PUBLIC, .name == "skip", .descriptor == "(J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/CheckedInputStream\0", "skip\0", "(J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChecksum](https://developer.android.com/reference/java/util/zip/CheckedInputStream.html#getChecksum())
        ///
        /// Required features: "java-util-zip-Checksum"
        #[cfg(any(feature = "all", all(feature = "java-util-zip-Checksum")))]
        pub fn getChecksum<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::zip::Checksum>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/CheckedInputStream", java.flags == PUBLIC, .name == "getChecksum", .descriptor == "()Ljava/util/zip/Checksum;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/CheckedInputStream\0", "getChecksum\0", "()Ljava/util/zip/Checksum;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
