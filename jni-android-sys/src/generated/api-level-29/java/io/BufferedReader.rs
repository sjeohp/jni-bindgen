// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-BufferedReader"))]
__jni_bindgen! {
    /// public class [BufferedReader](https://developer.android.com/reference/java/io/BufferedReader.html)
    ///
    /// Required feature: java-io-BufferedReader
    public class BufferedReader ("java/io/BufferedReader") extends crate::java::io::Reader {

        /// [BufferedReader](https://developer.android.com/reference/java/io/BufferedReader.html#BufferedReader(java.io.Reader,%20int))
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn new_Reader_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::Reader>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::BufferedReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/Reader;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "<init>\0", "(Ljava/io/Reader;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BufferedReader](https://developer.android.com/reference/java/io/BufferedReader.html#BufferedReader(java.io.Reader))
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn new_Reader<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::Reader>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::BufferedReader>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/Reader;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "<init>\0", "(Ljava/io/Reader;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/BufferedReader.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/BufferedReader.html#read(char%5B%5D,%20int,%20int))
        pub fn read_char_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "read", .descriptor == "([CII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "read\0", "([CII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [readLine](https://developer.android.com/reference/java/io/BufferedReader.html#readLine())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn readLine<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "readLine", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "readLine\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [skip](https://developer.android.com/reference/java/io/BufferedReader.html#skip(long))
        pub fn skip<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "skip", .descriptor == "(J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "skip\0", "(J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [ready](https://developer.android.com/reference/java/io/BufferedReader.html#ready())
        pub fn ready<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "ready", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "ready\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [markSupported](https://developer.android.com/reference/java/io/BufferedReader.html#markSupported())
        pub fn markSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "markSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "markSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mark](https://developer.android.com/reference/java/io/BufferedReader.html#mark(int))
        pub fn mark<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "mark", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "mark\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/java/io/BufferedReader.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/BufferedReader.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [lines](https://developer.android.com/reference/java/io/BufferedReader.html#lines())
        ///
        /// Required features: "java-util-stream-Stream"
        #[cfg(any(feature = "all", all(feature = "java-util-stream-Stream")))]
        pub fn lines<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::stream::Stream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedReader", java.flags == PUBLIC, .name == "lines", .descriptor == "()Ljava/util/stream/Stream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedReader\0", "lines\0", "()Ljava/util/stream/Stream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
