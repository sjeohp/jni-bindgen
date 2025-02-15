// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-ShortBuffer"))]
__jni_bindgen! {
    /// public class [ShortBuffer](https://developer.android.com/reference/java/nio/ShortBuffer.html)
    ///
    /// Required feature: java-nio-ShortBuffer
    public class ShortBuffer ("java/nio/ShortBuffer") extends crate::java::nio::Buffer, implements crate::java::lang::Comparable {

        // // Not emitting: Non-public method
        // /// [ShortBuffer](https://developer.android.com/reference/java/nio/ShortBuffer.html#ShortBuffer(int,%20int,%20int,%20int))
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/ShortBuffer", java.flags == (empty), .name == "<init>", .descriptor == "(IIII)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "<init>\0", "(IIII)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [allocate](https://developer.android.com/reference/java/nio/ShortBuffer.html#allocate(int))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn allocate<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | STATIC, .name == "allocate", .descriptor == "(I)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/ShortBuffer\0", "allocate\0", "(I)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [wrap](https://developer.android.com/reference/java/nio/ShortBuffer.html#wrap(short%5B%5D,%20int,%20int))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn wrap_short_array_int_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | STATIC, .name == "wrap", .descriptor == "([SII)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/ShortBuffer\0", "wrap\0", "([SII)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [wrap](https://developer.android.com/reference/java/nio/ShortBuffer.html#wrap(short%5B%5D))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn wrap_short_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | STATIC, .name == "wrap", .descriptor == "([S)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/ShortBuffer\0", "wrap\0", "([S)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [slice](https://developer.android.com/reference/java/nio/ShortBuffer.html#slice())
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn slice<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "slice", .descriptor == "()Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "slice\0", "()Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [duplicate](https://developer.android.com/reference/java/nio/ShortBuffer.html#duplicate())
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn duplicate<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "duplicate", .descriptor == "()Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "duplicate\0", "()Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [asReadOnlyBuffer](https://developer.android.com/reference/java/nio/ShortBuffer.html#asReadOnlyBuffer())
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn asReadOnlyBuffer<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "asReadOnlyBuffer", .descriptor == "()Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "asReadOnlyBuffer\0", "()Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/nio/ShortBuffer.html#get())
        pub fn get<'env>(&'env self) -> __jni_bindgen::std::result::Result<i16, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "()S"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "get\0", "()S\0");
                __jni_env.call_short_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/nio/ShortBuffer.html#put(short))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn put_short<'env>(&'env self, arg0: i16) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "put", .descriptor == "(S)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "put\0", "(S)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/nio/ShortBuffer.html#get(int))
        pub fn get_int<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<i16, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "get", .descriptor == "(I)S"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "get\0", "(I)S\0");
                __jni_env.call_short_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/nio/ShortBuffer.html#put(int,%20short))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn put_int_short<'env>(&'env self, arg0: i32, arg1: i16) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "put", .descriptor == "(IS)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "put\0", "(IS)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/nio/ShortBuffer.html#get(short%5B%5D,%20int,%20int))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn get_short_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "get", .descriptor == "([SII)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "get\0", "([SII)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/nio/ShortBuffer.html#get(short%5B%5D))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn get_short_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "get", .descriptor == "([S)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "get\0", "([S)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/nio/ShortBuffer.html#put(java.nio.ShortBuffer))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn put_ShortBuffer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ShortBuffer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "put", .descriptor == "(Ljava/nio/ShortBuffer;)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "put\0", "(Ljava/nio/ShortBuffer;)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/nio/ShortBuffer.html#put(short%5B%5D,%20int,%20int))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn put_short_array_int_int<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "put", .descriptor == "([SII)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "put\0", "([SII)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [put](https://developer.android.com/reference/java/nio/ShortBuffer.html#put(short%5B%5D))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn put_short_array<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | FINAL, .name == "put", .descriptor == "([S)Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "put\0", "([S)Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hasArray](https://developer.android.com/reference/java/nio/ShortBuffer.html#hasArray())
        pub fn hasArray<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | FINAL, .name == "hasArray", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "hasArray\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [array](https://developer.android.com/reference/java/nio/ShortBuffer.html#array())
        pub fn array<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ShortArray>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | FINAL, .name == "array", .descriptor == "()[S"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "array\0", "()[S\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [arrayOffset](https://developer.android.com/reference/java/nio/ShortBuffer.html#arrayOffset())
        pub fn arrayOffset<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | FINAL, .name == "arrayOffset", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "arrayOffset\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [position](https://developer.android.com/reference/java/nio/ShortBuffer.html#position(int))
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn position<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "position", .descriptor == "(I)Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "position\0", "(I)Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [limit](https://developer.android.com/reference/java/nio/ShortBuffer.html#limit(int))
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn limit<'env>(&'env self, arg0: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "limit", .descriptor == "(I)Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "limit\0", "(I)Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [mark](https://developer.android.com/reference/java/nio/ShortBuffer.html#mark())
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn mark<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "mark", .descriptor == "()Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "mark\0", "()Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [reset](https://developer.android.com/reference/java/nio/ShortBuffer.html#reset())
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn reset<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "reset", .descriptor == "()Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "reset\0", "()Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [clear](https://developer.android.com/reference/java/nio/ShortBuffer.html#clear())
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn clear<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "clear", .descriptor == "()Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "clear\0", "()Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [flip](https://developer.android.com/reference/java/nio/ShortBuffer.html#flip())
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn flip<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "flip", .descriptor == "()Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "flip\0", "()Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [rewind](https://developer.android.com/reference/java/nio/ShortBuffer.html#rewind())
        ///
        /// Required features: "java-nio-Buffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-Buffer")))]
        pub fn rewind<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::Buffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "rewind", .descriptor == "()Ljava/nio/Buffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "rewind\0", "()Ljava/nio/Buffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compact](https://developer.android.com/reference/java/nio/ShortBuffer.html#compact())
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn compact<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ShortBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "compact", .descriptor == "()Ljava/nio/ShortBuffer;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "compact\0", "()Ljava/nio/ShortBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isDirect](https://developer.android.com/reference/java/nio/ShortBuffer.html#isDirect())
        pub fn isDirect<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "isDirect", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "isDirect\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/nio/ShortBuffer.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/nio/ShortBuffer.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/nio/ShortBuffer.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareTo](https://developer.android.com/reference/java/nio/ShortBuffer.html#compareTo(java.nio.ShortBuffer))
        ///
        /// Required features: "java-nio-ShortBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ShortBuffer")))]
        pub fn compareTo_ShortBuffer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ShortBuffer>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC, .name == "compareTo", .descriptor == "(Ljava/nio/ShortBuffer;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "compareTo\0", "(Ljava/nio/ShortBuffer;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [order](https://developer.android.com/reference/java/nio/ShortBuffer.html#order())
        ///
        /// Required features: "java-nio-ByteOrder"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteOrder")))]
        pub fn order<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ByteOrder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | ABSTRACT, .name == "order", .descriptor == "()Ljava/nio/ByteOrder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "order\0", "()Ljava/nio/ByteOrder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [array](https://developer.android.com/reference/java/nio/ShortBuffer.html#array())
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn array<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "array", .descriptor == "()Ljava/lang/Object;"
        //     unsafe {
        //         let __jni_args = [];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "array\0", "()Ljava/lang/Object;\0");
        //         __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        // // Not emitting: Bridge method - type erasure
        // /// [compareTo](https://developer.android.com/reference/java/nio/ShortBuffer.html#compareTo(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn compareTo_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/ShortBuffer", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "compareTo", .descriptor == "(Ljava/lang/Object;)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/ShortBuffer\0", "compareTo\0", "(Ljava/lang/Object;)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
