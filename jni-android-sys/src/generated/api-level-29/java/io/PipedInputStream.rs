// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-PipedInputStream"))]
__jni_bindgen! {
    /// public class [PipedInputStream](https://developer.android.com/reference/java/io/PipedInputStream.html)
    ///
    /// Required feature: java-io-PipedInputStream
    public class PipedInputStream ("java/io/PipedInputStream") extends crate::java::io::InputStream {

        /// [PipedInputStream](https://developer.android.com/reference/java/io/PipedInputStream.html#PipedInputStream(java.io.PipedOutputStream))
        ///
        /// Required features: "java-io-PipedOutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-PipedOutputStream")))]
        pub fn new_PipedOutputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PipedOutputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/PipedOutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "<init>\0", "(Ljava/io/PipedOutputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PipedInputStream](https://developer.android.com/reference/java/io/PipedInputStream.html#PipedInputStream(java.io.PipedOutputStream,%20int))
        ///
        /// Required features: "java-io-PipedOutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-PipedOutputStream")))]
        pub fn new_PipedOutputStream_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PipedOutputStream>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/PipedOutputStream;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "<init>\0", "(Ljava/io/PipedOutputStream;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PipedInputStream](https://developer.android.com/reference/java/io/PipedInputStream.html#PipedInputStream())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [PipedInputStream](https://developer.android.com/reference/java/io/PipedInputStream.html#PipedInputStream(int))
        pub fn new_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::PipedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "<init>\0", "(I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [connect](https://developer.android.com/reference/java/io/PipedInputStream.html#connect(java.io.PipedOutputStream))
        ///
        /// Required features: "java-io-PipedOutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-PipedOutputStream")))]
        pub fn connect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::PipedOutputStream>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "connect", .descriptor == "(Ljava/io/PipedOutputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "connect\0", "(Ljava/io/PipedOutputStream;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [receive](https://developer.android.com/reference/java/io/PipedInputStream.html#receive(int))
        // fn receive<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/io/PipedInputStream", java.flags == PROTECTED | SYNCRONIZED, .name == "receive", .descriptor == "(I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "receive\0", "(I)V\0");
        //         __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [read](https://developer.android.com/reference/java/io/PipedInputStream.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/PipedInputStream.html#read(byte%5B%5D,%20int,%20int))
        pub fn read_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "read", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "read\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [available](https://developer.android.com/reference/java/io/PipedInputStream.html#available())
        pub fn available<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "available", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "available\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/PipedInputStream.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/PipedInputStream", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/PipedInputStream\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// protected static final [PIPE_SIZE](https://developer.android.com/reference/java/io/PipedInputStream.html#PIPE_SIZE)
        // pub const PIPE_SIZE : i32 = 1024;

        // // Not emitting: Non-public field
        // /// **get** protected [buffer](https://developer.android.com/reference/java/io/PipedInputStream.html#buffer)
        // pub fn buffer<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "buffer\0", "[B\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected [buffer](https://developer.android.com/reference/java/io/PipedInputStream.html#buffer)
        // pub fn set_buffer<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::ByteArray>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "buffer\0", "[B\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [in](https://developer.android.com/reference/java/io/PipedInputStream.html#in)
        // pub fn r#in<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "in\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [in](https://developer.android.com/reference/java/io/PipedInputStream.html#in)
        // pub fn set_in<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "in\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [out](https://developer.android.com/reference/java/io/PipedInputStream.html#out)
        // pub fn out<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "out\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [out](https://developer.android.com/reference/java/io/PipedInputStream.html#out)
        // pub fn set_out<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/PipedInputStream\0", "out\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }
    }
}
