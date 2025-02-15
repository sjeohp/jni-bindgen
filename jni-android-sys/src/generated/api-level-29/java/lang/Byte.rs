// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-lang-Byte"))]
__jni_bindgen! {
    /// public final class [Byte](https://developer.android.com/reference/java/lang/Byte.html)
    ///
    /// Required feature: java-lang-Byte
    public final class Byte ("java/lang/Byte") extends crate::java::lang::Number, implements crate::java::lang::Comparable {

        /// [Byte](https://developer.android.com/reference/java/lang/Byte.html#Byte(byte))
        pub fn new_byte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::Byte>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "<init>", .descriptor == "(B)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "<init>\0", "(B)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [Byte](https://developer.android.com/reference/java/lang/Byte.html#Byte(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::lang::Byte>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/lang/Byte.html#toString(byte))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString_byte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "toString", .descriptor == "(B)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "toString\0", "(B)Ljava/lang/String;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/lang/Byte.html#valueOf(byte))
        ///
        /// Required features: "java-lang-Byte"
        #[cfg(any(feature = "all", all(feature = "java-lang-Byte")))]
        pub fn valueOf_byte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Byte>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(B)Ljava/lang/Byte;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "valueOf\0", "(B)Ljava/lang/Byte;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseByte](https://developer.android.com/reference/java/lang/Byte.html#parseByte(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn parseByte_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "parseByte", .descriptor == "(Ljava/lang/String;I)B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "parseByte\0", "(Ljava/lang/String;I)B\0");
                __jni_env.call_static_byte_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [parseByte](https://developer.android.com/reference/java/lang/Byte.html#parseByte(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn parseByte_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "parseByte", .descriptor == "(Ljava/lang/String;)B"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "parseByte\0", "(Ljava/lang/String;)B\0");
                __jni_env.call_static_byte_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/lang/Byte.html#valueOf(java.lang.String,%20int))
        ///
        /// Required features: "java-lang-Byte", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Byte", feature = "java-lang-String")))]
        pub fn valueOf_String_int<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Byte>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;I)Ljava/lang/Byte;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "valueOf\0", "(Ljava/lang/String;I)Ljava/lang/Byte;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/lang/Byte.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-Byte", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Byte", feature = "java-lang-String")))]
        pub fn valueOf_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Byte>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/lang/Byte;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "valueOf\0", "(Ljava/lang/String;)Ljava/lang/Byte;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [decode](https://developer.android.com/reference/java/lang/Byte.html#decode(java.lang.String))
        ///
        /// Required features: "java-lang-Byte", "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-Byte", feature = "java-lang-String")))]
        pub fn decode<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Byte>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "decode", .descriptor == "(Ljava/lang/String;)Ljava/lang/Byte;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "decode\0", "(Ljava/lang/String;)Ljava/lang/Byte;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [byteValue](https://developer.android.com/reference/java/lang/Byte.html#byteValue())
        pub fn byteValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i8, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "byteValue", .descriptor == "()B"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "byteValue\0", "()B\0");
                __jni_env.call_byte_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [shortValue](https://developer.android.com/reference/java/lang/Byte.html#shortValue())
        pub fn shortValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i16, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "shortValue", .descriptor == "()S"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "shortValue\0", "()S\0");
                __jni_env.call_short_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [intValue](https://developer.android.com/reference/java/lang/Byte.html#intValue())
        pub fn intValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "intValue", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "intValue\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [longValue](https://developer.android.com/reference/java/lang/Byte.html#longValue())
        pub fn longValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "longValue", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "longValue\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [floatValue](https://developer.android.com/reference/java/lang/Byte.html#floatValue())
        pub fn floatValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<f32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "floatValue", .descriptor == "()F"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "floatValue\0", "()F\0");
                __jni_env.call_float_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [doubleValue](https://developer.android.com/reference/java/lang/Byte.html#doubleValue())
        pub fn doubleValue<'env>(&'env self) -> __jni_bindgen::std::result::Result<f64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "doubleValue", .descriptor == "()D"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "doubleValue\0", "()D\0");
                __jni_env.call_double_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/lang/Byte.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/lang/Byte.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/lang/Byte.html#hashCode(byte))
        pub fn hashCode_byte<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "hashCode", .descriptor == "(B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "hashCode\0", "(B)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/lang/Byte.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareTo](https://developer.android.com/reference/java/lang/Byte.html#compareTo(java.lang.Byte))
        ///
        /// Required features: "java-lang-Byte"
        #[cfg(any(feature = "all", all(feature = "java-lang-Byte")))]
        pub fn compareTo_Byte<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Byte>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC, .name == "compareTo", .descriptor == "(Ljava/lang/Byte;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "compareTo\0", "(Ljava/lang/Byte;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compare](https://developer.android.com/reference/java/lang/Byte.html#compare(byte,%20byte))
        pub fn compare<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8, arg1: i8) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "compare", .descriptor == "(BB)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "compare\0", "(BB)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toUnsignedInt](https://developer.android.com/reference/java/lang/Byte.html#toUnsignedInt(byte))
        pub fn toUnsignedInt<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "toUnsignedInt", .descriptor == "(B)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "toUnsignedInt\0", "(B)I\0");
                __jni_env.call_static_int_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toUnsignedLong](https://developer.android.com/reference/java/lang/Byte.html#toUnsignedLong(byte))
        pub fn toUnsignedLong<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: i8) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/lang/Byte", java.flags == PUBLIC | STATIC, .name == "toUnsignedLong", .descriptor == "(B)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/lang/Byte\0", "toUnsignedLong\0", "(B)J\0");
                __jni_env.call_static_long_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [compareTo](https://developer.android.com/reference/java/lang/Byte.html#compareTo(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn compareTo_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/lang/Byte", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "compareTo", .descriptor == "(Ljava/lang/Object;)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/lang/Byte\0", "compareTo\0", "(Ljava/lang/Object;)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// public static final [BYTES](https://developer.android.com/reference/java/lang/Byte.html#BYTES)
        pub const BYTES : i32 = 1;

        /// public static final [MAX_VALUE](https://developer.android.com/reference/java/lang/Byte.html#MAX_VALUE)
        pub const MAX_VALUE : i8 = 127;

        /// public static final [MIN_VALUE](https://developer.android.com/reference/java/lang/Byte.html#MIN_VALUE)
        pub const MIN_VALUE : i8 = -128;

        /// public static final [SIZE](https://developer.android.com/reference/java/lang/Byte.html#SIZE)
        pub const SIZE : i32 = 8;

        /// **get** public static final [TYPE](https://developer.android.com/reference/java/lang/Byte.html#TYPE)
        ///
        /// Required feature: java-lang-Class
        #[cfg(any(feature = "all", feature = "java-lang-Class"))]
        pub fn TYPE<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::Class>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/lang/Byte\0", "TYPE\0", "Ljava/lang/Class;\0");
                env.get_static_object_field(class, field)
            }
        }
    }
}
