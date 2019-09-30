// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-io-BufferedInputStream"))]
__jni_bindgen! {
    /// public class [BufferedInputStream](https://developer.android.com/reference/java/io/BufferedInputStream.html)
    ///
    /// Required feature: java-io-BufferedInputStream
    public class BufferedInputStream ("java/io/BufferedInputStream") extends crate::java::io::FilterInputStream {

        /// [BufferedInputStream](https://developer.android.com/reference/java/io/BufferedInputStream.html#BufferedInputStream(java.io.InputStream))
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn new_InputStream<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::BufferedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "<init>\0", "(Ljava/io/InputStream;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [BufferedInputStream](https://developer.android.com/reference/java/io/BufferedInputStream.html#BufferedInputStream(java.io.InputStream,%20int))
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn new_InputStream_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::io::InputStream>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::io::BufferedInputStream>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/io/InputStream;I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "<init>\0", "(Ljava/io/InputStream;I)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/BufferedInputStream.html#read())
        pub fn read<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "read", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "read\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [read](https://developer.android.com/reference/java/io/BufferedInputStream.html#read(byte%5B%5D,%20int,%20int))
        pub fn read_byte_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "read", .descriptor == "([BII)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "read\0", "([BII)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [skip](https://developer.android.com/reference/java/io/BufferedInputStream.html#skip(long))
        pub fn skip<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "skip", .descriptor == "(J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "skip\0", "(J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [available](https://developer.android.com/reference/java/io/BufferedInputStream.html#available())
        pub fn available<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "available", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "available\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mark](https://developer.android.com/reference/java/io/BufferedInputStream.html#mark(int))
        pub fn mark<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "mark", .descriptor == "(I)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "mark\0", "(I)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/java/io/BufferedInputStream.html#reset())
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC | SYNCRONIZED, .name == "reset", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "reset\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [markSupported](https://developer.android.com/reference/java/io/BufferedInputStream.html#markSupported())
        pub fn markSupported<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC, .name == "markSupported", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "markSupported\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/java/io/BufferedInputStream.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/io/BufferedInputStream", java.flags == PUBLIC, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/io/BufferedInputStream\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public field
        // /// **get** protected volatile [buf](https://developer.android.com/reference/java/io/BufferedInputStream.html#buf)
        // pub fn buf<'env>(&'env self) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray>> {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "buf\0", "[B\0");
        //         env.get_object_field(class, field)
        //     }
        // }

        // /// **set** protected volatile [buf](https://developer.android.com/reference/java/io/BufferedInputStream.html#buf)
        // pub fn set_buf<'env, 'obj>(&'env self, value: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj __jni_bindgen::ByteArray>>) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "buf\0", "[B\0");
        //         env.set_object_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [count](https://developer.android.com/reference/java/io/BufferedInputStream.html#count)
        // pub fn count<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "count\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [count](https://developer.android.com/reference/java/io/BufferedInputStream.html#count)
        // pub fn set_count<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "count\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [marklimit](https://developer.android.com/reference/java/io/BufferedInputStream.html#marklimit)
        // pub fn marklimit<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "marklimit\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [marklimit](https://developer.android.com/reference/java/io/BufferedInputStream.html#marklimit)
        // pub fn set_marklimit<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "marklimit\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [markpos](https://developer.android.com/reference/java/io/BufferedInputStream.html#markpos)
        // pub fn markpos<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "markpos\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [markpos](https://developer.android.com/reference/java/io/BufferedInputStream.html#markpos)
        // pub fn set_markpos<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "markpos\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }

        // // Not emitting: Non-public field
        // /// **get** protected [pos](https://developer.android.com/reference/java/io/BufferedInputStream.html#pos)
        // pub fn pos<'env>(&'env self) -> i32 {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "pos\0", "I\0");
        //         env.get_int_field(class, field)
        //     }
        // }

        // /// **set** protected [pos](https://developer.android.com/reference/java/io/BufferedInputStream.html#pos)
        // pub fn set_pos<'env>(&'env self, value: i32) {
        //     unsafe {
        //         let env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (class, field) = env.require_class_field("java/io/BufferedInputStream\0", "pos\0", "I\0");
        //         env.set_int_field(class, field, value)
        //     }
        // }
    }
}
