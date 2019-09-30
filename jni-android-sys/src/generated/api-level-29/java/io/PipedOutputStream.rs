// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-PipedOutputStream"))]
__jni_bindgen! {
    /// public class [PipedOutputStream](https://developer.android.com/reference/java/io/PipedOutputStream.html)
    ///
    /// Required feature: java-io-PipedOutputStream
    public class PipedOutputStream ("java/io/PipedOutputStream") extends crate::java::io::OutputStream {

        /// [PipedOutputStream](https://developer.android.com/reference/java/io/PipedOutputStream.html#PipedOutputStream(java.io.PipedInputStream))
        ///
        /// Required features: "java-io-PipedInputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-PipedInputStream")))]
        pub fn new_PipedInputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PipedInputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/PipedInputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "<init>\0", "(Ljava/io/PipedInputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PipedOutputStream](https://developer.android.com/reference/java/io/PipedOutputStream.html#PipedOutputStream())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedOutputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/java/io/PipedOutputStream.html#connect(java.io.PipedInputStream))
        ///
        /// Required features: "java-io-PipedInputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-PipedInputStream")))]
        pub fn connect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PipedInputStream>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "connect", .descriptor == "(Ljava/io/PipedInputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "connect\0", "(Ljava/io/PipedInputStream;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/io/PipedOutputStream.html#write(int))
        pub fn write_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC, .name == "write", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "write\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/io/PipedOutputStream.html#write(byte%5B%5D,%20int,%20int))
        pub fn write_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC, .name == "write", .descriptor == "([BII)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "write\0", "([BII)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [flush](https://developer.android.com/reference/java/io/PipedOutputStream.html#flush())
        pub fn flush<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "flush", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "flush\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/PipedOutputStream.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedOutputStream", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedOutputStream\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
