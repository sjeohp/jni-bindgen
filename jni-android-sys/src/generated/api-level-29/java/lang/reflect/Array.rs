// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-reflect-Array"))]
__jni_bindgen! {
    /// public final class [Array](https://developer.android.com/reference/java/lang/reflect/Array.html)
    ///
    /// Required feature: java-lang-reflect-Array
    public final class Array ("java/lang/reflect/Array") extends crate::java::lang::Object {

        // // Not emitting: Non-public method
        // /// [Array](https://developer.android.com/reference/java/lang/reflect/Array.html#Array())
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::reflect::Array>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/lang/reflect/Array", java.flags == (empty), .name == "<init>", .descriptor == "()V"
        //     unsafe {
        //         let __jni_args = [];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/reflect/Array\0", "<init>\0", "()V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [newInstance](https://developer.android.com/reference/java/lang/reflect/Array.html#newInstance(java.lang.Class,%20int))
        ///
        /// Required features: "java-lang-Class", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-Object")))]
        pub fn newInstance_Class_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "newInstance", .descriptor == "(Ljava/lang/Class;I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "newInstance\0", "(Ljava/lang/Class;I)Ljava/lang/Object;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newInstance](https://developer.android.com/reference/java/lang/reflect/Array.html#newInstance(java.lang.Class,%20int...))
        ///
        /// Required features: "java-lang-Class", "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Class", feature = "java-lang-Object")))]
        pub fn newInstance_Class_int_array<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Class>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC | VARARGS, .name == "newInstance", .descriptor == "(Ljava/lang/Class;[I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "newInstance\0", "(Ljava/lang/Class;[I)Ljava/lang/Object;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLength](https://developer.android.com/reference/java/lang/reflect/Array.html#getLength(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getLength<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getLength", .descriptor == "(Ljava/lang/Object;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getLength\0", "(Ljava/lang/Object;)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [get](https://developer.android.com/reference/java/lang/reflect/Array.html#get(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn get<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Object>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "get", .descriptor == "(Ljava/lang/Object;I)Ljava/lang/Object;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "get\0", "(Ljava/lang/Object;I)Ljava/lang/Object;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getBoolean](https://developer.android.com/reference/java/lang/reflect/Array.html#getBoolean(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getBoolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getBoolean", .descriptor == "(Ljava/lang/Object;I)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getBoolean\0", "(Ljava/lang/Object;I)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getByte](https://developer.android.com/reference/java/lang/reflect/Array.html#getByte(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getByte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getByte", .descriptor == "(Ljava/lang/Object;I)B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getByte\0", "(Ljava/lang/Object;I)B\0");
                __jni_env.call_static_byte_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getChar](https://developer.android.com/reference/java/lang/reflect/Array.html#getChar(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getChar<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::jchar, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getChar", .descriptor == "(Ljava/lang/Object;I)C"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getChar\0", "(Ljava/lang/Object;I)C\0");
                __jni_env.call_static_char_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getShort](https://developer.android.com/reference/java/lang/reflect/Array.html#getShort(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getShort<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<i16, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getShort", .descriptor == "(Ljava/lang/Object;I)S"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getShort\0", "(Ljava/lang/Object;I)S\0");
                __jni_env.call_static_short_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getInt](https://developer.android.com/reference/java/lang/reflect/Array.html#getInt(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getInt<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getInt", .descriptor == "(Ljava/lang/Object;I)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getInt\0", "(Ljava/lang/Object;I)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getLong](https://developer.android.com/reference/java/lang/reflect/Array.html#getLong(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getLong<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getLong", .descriptor == "(Ljava/lang/Object;I)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getLong\0", "(Ljava/lang/Object;I)J\0");
                __jni_env.call_static_long_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getFloat](https://developer.android.com/reference/java/lang/reflect/Array.html#getFloat(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getFloat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getFloat", .descriptor == "(Ljava/lang/Object;I)F"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getFloat\0", "(Ljava/lang/Object;I)F\0");
                __jni_env.call_static_float_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getDouble](https://developer.android.com/reference/java/lang/reflect/Array.html#getDouble(java.lang.Object,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn getDouble<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "getDouble", .descriptor == "(Ljava/lang/Object;I)D"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "getDouble\0", "(Ljava/lang/Object;I)D\0");
                __jni_env.call_static_double_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [set](https://developer.android.com/reference/java/lang/reflect/Array.html#set(java.lang.Object,%20int,%20java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn set<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "set", .descriptor == "(Ljava/lang/Object;ILjava/lang/Object;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "set\0", "(Ljava/lang/Object;ILjava/lang/Object;)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setBoolean](https://developer.android.com/reference/java/lang/reflect/Array.html#setBoolean(java.lang.Object,%20int,%20boolean))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setBoolean<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: bool) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setBoolean", .descriptor == "(Ljava/lang/Object;IZ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setBoolean\0", "(Ljava/lang/Object;IZ)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setByte](https://developer.android.com/reference/java/lang/reflect/Array.html#setByte(java.lang.Object,%20int,%20byte))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setByte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: i8) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setByte", .descriptor == "(Ljava/lang/Object;IB)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setByte\0", "(Ljava/lang/Object;IB)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setChar](https://developer.android.com/reference/java/lang/reflect/Array.html#setChar(java.lang.Object,%20int,%20char))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setChar<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: __jni_bindgen::jchar) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setChar", .descriptor == "(Ljava/lang/Object;IC)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setChar\0", "(Ljava/lang/Object;IC)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setShort](https://developer.android.com/reference/java/lang/reflect/Array.html#setShort(java.lang.Object,%20int,%20short))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setShort<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: i16) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setShort", .descriptor == "(Ljava/lang/Object;IS)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setShort\0", "(Ljava/lang/Object;IS)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setInt](https://developer.android.com/reference/java/lang/reflect/Array.html#setInt(java.lang.Object,%20int,%20int))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setInt<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: i32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setInt", .descriptor == "(Ljava/lang/Object;II)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setInt\0", "(Ljava/lang/Object;II)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setLong](https://developer.android.com/reference/java/lang/reflect/Array.html#setLong(java.lang.Object,%20int,%20long))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setLong<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setLong", .descriptor == "(Ljava/lang/Object;IJ)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setLong\0", "(Ljava/lang/Object;IJ)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setFloat](https://developer.android.com/reference/java/lang/reflect/Array.html#setFloat(java.lang.Object,%20int,%20float))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setFloat<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: f32) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setFloat", .descriptor == "(Ljava/lang/Object;IF)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setFloat\0", "(Ljava/lang/Object;IF)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setDouble](https://developer.android.com/reference/java/lang/reflect/Array.html#setDouble(java.lang.Object,%20int,%20double))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn setDouble<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>, arg1: i32, arg2: f64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/reflect/Array", java.flags == PUBLIC | STATIC, .name == "setDouble", .descriptor == "(Ljava/lang/Object;ID)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1), __jni_bindgen::AsJValue::as_jvalue(&arg2)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/reflect/Array\0", "setDouble\0", "(Ljava/lang/Object;ID)V\0");
                __jni_env.call_static_void_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
