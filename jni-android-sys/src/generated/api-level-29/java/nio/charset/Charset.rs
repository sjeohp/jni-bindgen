// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-nio-charset-Charset"))]
__jni_bindgen! {
    /// public class [Charset](https://developer.android.com/reference/java/nio/charset/Charset.html)
    ///
    /// Required feature: java-nio-charset-Charset
    public class Charset ("java/nio/charset/Charset") extends crate::java::lang::Object, implements crate::java::lang::Comparable {

        // // Not emitting: Non-public method
        // /// [Charset](https://developer.android.com/reference/java/nio/charset/Charset.html#Charset(java.lang.String,%20java.lang.String%5B%5D))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::nio::charset::Charset>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/charset/Charset", java.flags == PROTECTED, .name == "<init>", .descriptor == "(Ljava/lang/String;[Ljava/lang/String;)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "<init>\0", "(Ljava/lang/String;[Ljava/lang/String;)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// [isSupported](https://developer.android.com/reference/java/nio/charset/Charset.html#isSupported(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn isSupported<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | STATIC, .name == "isSupported", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/charset/Charset\0", "isSupported\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_static_boolean_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [forName](https://developer.android.com/reference/java/nio/charset/Charset.html#forName(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-charset-Charset")))]
        pub fn forName<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::charset::Charset>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | STATIC, .name == "forName", .descriptor == "(Ljava/lang/String;)Ljava/nio/charset/Charset;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/charset/Charset\0", "forName\0", "(Ljava/lang/String;)Ljava/nio/charset/Charset;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [availableCharsets](https://developer.android.com/reference/java/nio/charset/Charset.html#availableCharsets())
        ///
        /// Required features: "java-util-SortedMap"
        #[cfg(any(feature = "all", all(feature = "java-util-SortedMap")))]
        pub fn availableCharsets<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::SortedMap>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | STATIC, .name == "availableCharsets", .descriptor == "()Ljava/util/SortedMap;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/charset/Charset\0", "availableCharsets\0", "()Ljava/util/SortedMap;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [defaultCharset](https://developer.android.com/reference/java/nio/charset/Charset.html#defaultCharset())
        ///
        /// Required features: "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-nio-charset-Charset")))]
        pub fn defaultCharset<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::charset::Charset>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | STATIC, .name == "defaultCharset", .descriptor == "()Ljava/nio/charset/Charset;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/nio/charset/Charset\0", "defaultCharset\0", "()Ljava/nio/charset/Charset;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [name](https://developer.android.com/reference/java/nio/charset/Charset.html#name())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn name<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "name", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "name\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [aliases](https://developer.android.com/reference/java/nio/charset/Charset.html#aliases())
        ///
        /// Required features: "java-util-Set"
        #[cfg(any(feature = "all", all(feature = "java-util-Set")))]
        pub fn aliases<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::util::Set>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "aliases", .descriptor == "()Ljava/util/Set;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "aliases\0", "()Ljava/util/Set;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [displayName](https://developer.android.com/reference/java/nio/charset/Charset.html#displayName())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn displayName<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC, .name == "displayName", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "displayName\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [isRegistered](https://developer.android.com/reference/java/nio/charset/Charset.html#isRegistered())
        pub fn isRegistered<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "isRegistered", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "isRegistered\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [displayName](https://developer.android.com/reference/java/nio/charset/Charset.html#displayName(java.util.Locale))
        ///
        /// Required features: "java-lang-String", "java-util-Locale"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-util-Locale")))]
        pub fn displayName_Locale<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Locale>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC, .name == "displayName", .descriptor == "(Ljava/util/Locale;)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "displayName\0", "(Ljava/util/Locale;)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [contains](https://developer.android.com/reference/java/nio/charset/Charset.html#contains(java.nio.charset.Charset))
        ///
        /// Required features: "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-nio-charset-Charset")))]
        pub fn contains<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::Charset>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | ABSTRACT, .name == "contains", .descriptor == "(Ljava/nio/charset/Charset;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "contains\0", "(Ljava/nio/charset/Charset;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newDecoder](https://developer.android.com/reference/java/nio/charset/Charset.html#newDecoder())
        ///
        /// Required features: "java-nio-charset-CharsetDecoder"
        #[cfg(any(feature = "all", all(feature = "java-nio-charset-CharsetDecoder")))]
        pub fn newDecoder<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::charset::CharsetDecoder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | ABSTRACT, .name == "newDecoder", .descriptor == "()Ljava/nio/charset/CharsetDecoder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "newDecoder\0", "()Ljava/nio/charset/CharsetDecoder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [newEncoder](https://developer.android.com/reference/java/nio/charset/Charset.html#newEncoder())
        ///
        /// Required features: "java-nio-charset-CharsetEncoder"
        #[cfg(any(feature = "all", all(feature = "java-nio-charset-CharsetEncoder")))]
        pub fn newEncoder<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::charset::CharsetEncoder>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | ABSTRACT, .name == "newEncoder", .descriptor == "()Ljava/nio/charset/CharsetEncoder;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "newEncoder\0", "()Ljava/nio/charset/CharsetEncoder;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [canEncode](https://developer.android.com/reference/java/nio/charset/Charset.html#canEncode())
        pub fn canEncode<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC, .name == "canEncode", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "canEncode\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [decode](https://developer.android.com/reference/java/nio/charset/Charset.html#decode(java.nio.ByteBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer", "java-nio-CharBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer", feature = "java-nio-CharBuffer")))]
        pub fn decode<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::ByteBuffer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::CharBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "decode", .descriptor == "(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "decode\0", "(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [encode](https://developer.android.com/reference/java/nio/charset/Charset.html#encode(java.nio.CharBuffer))
        ///
        /// Required features: "java-nio-ByteBuffer", "java-nio-CharBuffer"
        #[cfg(any(feature = "all", all(feature = "java-nio-ByteBuffer", feature = "java-nio-CharBuffer")))]
        pub fn encode_CharBuffer<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::CharBuffer>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ByteBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "encode", .descriptor == "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "encode\0", "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [encode](https://developer.android.com/reference/java/nio/charset/Charset.html#encode(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-nio-ByteBuffer"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-nio-ByteBuffer")))]
        pub fn encode_String<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::nio::ByteBuffer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "encode", .descriptor == "(Ljava/lang/String;)Ljava/nio/ByteBuffer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "encode\0", "(Ljava/lang/String;)Ljava/nio/ByteBuffer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [compareTo](https://developer.android.com/reference/java/nio/charset/Charset.html#compareTo(java.nio.charset.Charset))
        ///
        /// Required features: "java-nio-charset-Charset"
        #[cfg(any(feature = "all", all(feature = "java-nio-charset-Charset")))]
        pub fn compareTo_Charset<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::nio::charset::Charset>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "compareTo", .descriptor == "(Ljava/nio/charset/Charset;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "compareTo\0", "(Ljava/nio/charset/Charset;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [hashCode](https://developer.android.com/reference/java/nio/charset/Charset.html#hashCode())
        pub fn hashCode<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "hashCode", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "hashCode\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [equals](https://developer.android.com/reference/java/nio/charset/Charset.html#equals(java.lang.Object))
        ///
        /// Required features: "java-lang-Object"
        #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        pub fn equals<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "equals", .descriptor == "(Ljava/lang/Object;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "equals\0", "(Ljava/lang/Object;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [toString](https://developer.android.com/reference/java/nio/charset/Charset.html#toString())
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn toString<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | FINAL, .name == "toString", .descriptor == "()Ljava/lang/String;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "toString\0", "()Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Bridge method - type erasure
        // /// [compareTo](https://developer.android.com/reference/java/nio/charset/Charset.html#compareTo(java.lang.Object))
        // ///
        // /// Required features: "java-lang-Object"
        // #[cfg(any(feature = "all", all(feature = "java-lang-Object")))]
        // pub fn compareTo_Object<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::Object>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/nio/charset/Charset", java.flags == PUBLIC | BRIDGE | SYNTHETIC, .name == "compareTo", .descriptor == "(Ljava/lang/Object;)I"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
        //         let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/nio/charset/Charset\0", "compareTo\0", "(Ljava/lang/Object;)I\0");
        //         __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
        //     }
        // }
    }
}
