// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-util-zip-GZIPOutputStream"))]
__jni_bindgen! {
    /// public class [GZIPOutputStream](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html)
    ///
    /// Required feature: java-util-zip-GZIPOutputStream
    public class GZIPOutputStream ("java/util/zip/GZIPOutputStream") extends crate::java::util::zip::DeflaterOutputStream {

        /// [GZIPOutputStream](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#GZIPOutputStream(java.io.OutputStream,%20int))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::GZIPOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GZIPOutputStream](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#GZIPOutputStream(java.io.OutputStream,%20int,%20boolean))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream_int_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: i32, arg2: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::GZIPOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;IZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;IZ)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GZIPOutputStream](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#GZIPOutputStream(java.io.OutputStream))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::GZIPOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [GZIPOutputStream](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#GZIPOutputStream(java.io.OutputStream,%20boolean))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream_boolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: bool) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::util::zip::GZIPOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;Z)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "<init>\0", "(Ljava/io/OutputStream;Z)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#write(byte%5B%5D,%20int,%20int))
        pub fn write<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "write", .descriptor == "([BII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "write\0", "([BII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [finish](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#finish())
        pub fn finish<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/util/zip/GZIPOutputStream", java.flags == PUBLIC, .name == "finish", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/util/zip/GZIPOutputStream\0", "finish\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected [crc](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#crc)
        // ///
        // /// Required feature: java-util-zip-CRC32
        // #[cfg(any(feature = "all", feature = "java-util-zip-CRC32"))]
        // pub fn crc<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::zip::CRC32>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/util/zip/GZIPOutputStream\0", "crc\0", "Ljava/util/zip/CRC32;\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [crc](https://developer.android.com/reference/java/util/zip/GZIPOutputStream.html#crc)
        // ///
        // /// Required feature: java-util-zip-CRC32
        // #[cfg(any(feature = "all", feature = "java-util-zip-CRC32"))]
        // pub fn set_crc<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj crate::java::util::zip::CRC32>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/util/zip/GZIPOutputStream\0", "crc\0", "Ljava/util/zip/CRC32;\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }
    }
}
