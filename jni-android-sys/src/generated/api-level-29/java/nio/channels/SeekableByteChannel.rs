// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-channels-SeekableByteChannel"))]
__jni_bindgen! {
    /// public interface [SeekableByteChannel](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html)
    ///
    /// Required feature: java-nio-channels-SeekableByteChannel
    public interface SeekableByteChannel ("java/nio/channels/SeekableByteChannel") extends crate::java::lang::Object, implements crate::java::nio::channels::ByteChannel {

        /// [read](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#read(java.nio.ByteBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn read<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "read", .descriptor == "(Ljava/nio/ByteBuffer;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "read\0", "(Ljava/nio/ByteBuffer;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [write](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#write(java.nio.ByteBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer")))]
        pub fn write<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "write", .descriptor == "(Ljava/nio/ByteBuffer;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "write\0", "(Ljava/nio/ByteBuffer;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [position](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#position())
        pub fn position<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "position", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "position\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [position](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#position(long))
        ///
        /// Required features: "java-nio-channels-SeekableByteChannel"
        #[cfg(any(feature = "all", all(feature = "java-nio-channels-SeekableByteChannel")))]
        pub fn position_long<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::SeekableByteChannel>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "position", .descriptor == "(J)Ljava/nio/channels/SeekableByteChannel;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "position\0", "(J)Ljava/nio/channels/SeekableByteChannel;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [size](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#size())
        pub fn size<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "size", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "size\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [truncate](https://developer.android.com/reference/java/nio/channels/SeekableByteChannel.html#truncate(long))
        ///
        /// Required features: "java-nio-channels-SeekableByteChannel"
        #[cfg(any(feature = "all", all(feature = "java-nio-channels-SeekableByteChannel")))]
        pub fn truncate<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::channels::SeekableByteChannel>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/channels/SeekableByteChannel", java.flags == PUBLIC | ABSTRACT, .name == "truncate", .descriptor == "(J)Ljava/nio/channels/SeekableByteChannel;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/channels/SeekableByteChannel\0", "truncate\0", "(J)Ljava/nio/channels/SeekableByteChannel;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
