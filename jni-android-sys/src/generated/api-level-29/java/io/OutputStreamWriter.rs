// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-OutputStreamWriter"))]
__jni_bindgen! {
    /// public class [OutputStreamWriter](https://developer.android.com/reference/java/io/OutputStreamWriter.html)
    ///
    /// Required feature: java-io-OutputStreamWriter
    public class OutputStreamWriter ("java/io/OutputStreamWriter") extends crate::java::io::Writer {

        /// [OutputStreamWriter](https://developer.android.com/reference/java/io/OutputStreamWriter.html#OutputStreamWriter(java.io.OutputStream,%20java.lang.String))
        ///
        /// Required features: "java-io-OutputStream", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream", feature = "java-lang-String")))]
        pub fn new_OutputStream_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::OutputStreamWriter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "<init>\0", "(Ljava/io/OutputStream;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [OutputStreamWriter](https://developer.android.com/reference/java/io/OutputStreamWriter.html#OutputStreamWriter(java.io.OutputStream))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn new_OutputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::OutputStreamWriter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "<init>\0", "(Ljava/io/OutputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [OutputStreamWriter](https://developer.android.com/reference/java/io/OutputStreamWriter.html#OutputStreamWriter(java.io.OutputStream,%20java.nio.charset.Charset))
        ///
        /// Required features: "java-io-OutputStream", "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream", feature = "java-nio-charset-Charset")))]
        pub fn new_OutputStream_Charset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::Charset>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::OutputStreamWriter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "<init>\0", "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [OutputStreamWriter](https://developer.android.com/reference/java/io/OutputStreamWriter.html#OutputStreamWriter(java.io.OutputStream,%20java.nio.charset.CharsetEncoder))
        ///
        /// Required features: "java-io-OutputStream", "java-nio-charset-CharsetEncoder"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream", feature = "java-nio-charset-CharsetEncoder")))]
        pub fn new_OutputStream_CharsetEncoder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::OutputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::CharsetEncoder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::OutputStreamWriter>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "<init>\0", "(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncoding](https://developer.android.com/reference/java/io/OutputStreamWriter.html#getEncoding())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEncoding<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "getEncoding", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "getEncoding\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/io/OutputStreamWriter.html#write(int))
        pub fn write_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "write", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "write\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/io/OutputStreamWriter.html#write(char%5B%5D,%20int,%20int))
        pub fn write_char_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "write", .descriptor == "([CII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "write\0", "([CII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/io/OutputStreamWriter.html#write(java.lang.String,%20int,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn write_String_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "write", .descriptor == "(Ljava/lang/String;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "write\0", "(Ljava/lang/String;II)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [flush](https://developer.android.com/reference/java/io/OutputStreamWriter.html#flush())
        pub fn flush<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "flush", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "flush\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/OutputStreamWriter.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/OutputStreamWriter", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/OutputStreamWriter\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
