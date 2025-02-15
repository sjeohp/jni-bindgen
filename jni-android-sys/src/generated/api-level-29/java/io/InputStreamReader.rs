// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-InputStreamReader"))]
__jni_bindgen! {
    /// public class [InputStreamReader](https://developer.android.com/reference/java/io/InputStreamReader.html)
    ///
    /// Required feature: java-io-InputStreamReader
    public class InputStreamReader ("java/io/InputStreamReader") extends crate::java::io::Reader {

        /// [InputStreamReader](https://developer.android.com/reference/java/io/InputStreamReader.html#InputStreamReader(java.io.InputStream))
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn new_InputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InputStreamReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "<init>\0", "(Ljava/io/InputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InputStreamReader](https://developer.android.com/reference/java/io/InputStreamReader.html#InputStreamReader(java.io.InputStream,%20java.lang.String))
        ///
        /// Required features: "java-io-InputStream", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-lang-String")))]
        pub fn new_InputStream_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InputStreamReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "<init>\0", "(Ljava/io/InputStream;Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InputStreamReader](https://developer.android.com/reference/java/io/InputStreamReader.html#InputStreamReader(java.io.InputStream,%20java.nio.charset.Charset))
        ///
        /// Required features: "java-io-InputStream", "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-nio-charset-Charset")))]
        pub fn new_InputStream_Charset<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::Charset>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InputStreamReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;Ljava/nio/charset/Charset;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "<init>\0", "(Ljava/io/InputStream;Ljava/nio/charset/Charset;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [InputStreamReader](https://developer.android.com/reference/java/io/InputStreamReader.html#InputStreamReader(java.io.InputStream,%20java.nio.charset.CharsetDecoder))
        ///
        /// Required features: "java-io-InputStream", "java-nio-charset-CharsetDecoder"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream", feature = "java-nio-charset-CharsetDecoder")))]
        pub fn new_InputStream_CharsetDecoder<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::CharsetDecoder>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::InputStreamReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;Ljava/nio/charset/CharsetDecoder;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "<init>\0", "(Ljava/io/InputStream;Ljava/nio/charset/CharsetDecoder;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getEncoding](https://developer.android.com/reference/java/io/InputStreamReader.html#getEncoding())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getEncoding<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "getEncoding", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "getEncoding\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/InputStreamReader.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/InputStreamReader.html#read(char%5B%5D,%20int,%20int))
        pub fn read_char_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "read", .descriptor == "([CII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "read\0", "([CII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ready](https://developer.android.com/reference/java/io/InputStreamReader.html#ready())
        pub fn ready<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "ready", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "ready\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/InputStreamReader.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/InputStreamReader", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/InputStreamReader\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
