// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-sql-Clob"))]
__jni_bindgen! {
    /// public interface [Clob](https://developer.android.com/reference/java/sql/Clob.html)
    ///
    /// Required feature: java-sql-Clob
    public interface Clob ("java/sql/Clob") extends crate::java::lang::Object {

        /// [length](https://developer.android.com/reference/java/sql/Clob.html#length())
        pub fn length<'env>(&'env self) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "length", .descriptor == "()J"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "length\0", "()J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSubString](https://developer.android.com/reference/java/sql/Clob.html#getSubString(long,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn getSubString<'env>(&'env self, arg0: i64, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::lang::String>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "getSubString", .descriptor == "(JI)Ljava/lang/String;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "getSubString\0", "(JI)Ljava/lang/String;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCharacterStream](https://developer.android.com/reference/java/sql/Clob.html#getCharacterStream())
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn getCharacterStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::Reader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "getCharacterStream", .descriptor == "()Ljava/io/Reader;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "getCharacterStream\0", "()Ljava/io/Reader;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getAsciiStream](https://developer.android.com/reference/java/sql/Clob.html#getAsciiStream())
        ///
        /// Required features: "java-io-InputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-InputStream")))]
        pub fn getAsciiStream<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::InputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "getAsciiStream", .descriptor == "()Ljava/io/InputStream;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "getAsciiStream\0", "()Ljava/io/InputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [position](https://developer.android.com/reference/java/sql/Clob.html#position(java.lang.String,%20long))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn position_String_long<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "position", .descriptor == "(Ljava/lang/String;J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "position\0", "(Ljava/lang/String;J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [position](https://developer.android.com/reference/java/sql/Clob.html#position(java.sql.Clob,%20long))
        ///
        /// Required features: "java-sql-Clob"
        #[cfg(any(feature = "all", all(feature = "java-sql-Clob")))]
        pub fn position_Clob_long<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::sql::Clob>>, arg1: i64) -> __jni_bindgen::std::result::Result<i64, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "position", .descriptor == "(Ljava/sql/Clob;J)J"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "position\0", "(Ljava/sql/Clob;J)J\0");
                __jni_env.call_long_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setString](https://developer.android.com/reference/java/sql/Clob.html#setString(long,%20java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setString_long_String<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "setString", .descriptor == "(JLjava/lang/String;)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "setString\0", "(JLjava/lang/String;)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setString](https://developer.android.com/reference/java/sql/Clob.html#setString(long,%20java.lang.String,%20int,%20int))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn setString_long_String_int_int<'env>(&'env self, arg0: i64, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg2: i32, arg3: i32) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "setString", .descriptor == "(JLjava/lang/String;II)I"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2), __jni_bindgen::AsJValue::as_jvalue(&arg3)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "setString\0", "(JLjava/lang/String;II)I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setAsciiStream](https://developer.android.com/reference/java/sql/Clob.html#setAsciiStream(long))
        ///
        /// Required features: "java-io-OutputStream"
        #[cfg(any(feature = "all", all(feature = "java-io-OutputStream")))]
        pub fn setAsciiStream<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::OutputStream>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "setAsciiStream", .descriptor == "(J)Ljava/io/OutputStream;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "setAsciiStream\0", "(J)Ljava/io/OutputStream;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [setCharacterStream](https://developer.android.com/reference/java/sql/Clob.html#setCharacterStream(long))
        ///
        /// Required features: "java-io-Writer"
        #[cfg(any(feature = "all", all(feature = "java-io-Writer")))]
        pub fn setCharacterStream<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::Writer>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "setCharacterStream", .descriptor == "(J)Ljava/io/Writer;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "setCharacterStream\0", "(J)Ljava/io/Writer;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [truncate](https://developer.android.com/reference/java/sql/Clob.html#truncate(long))
        pub fn truncate<'env>(&'env self, arg0: i64) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "truncate", .descriptor == "(J)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "truncate\0", "(J)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [free](https://developer.android.com/reference/java/sql/Clob.html#free())
        pub fn free<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "free", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "free\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getCharacterStream](https://developer.android.com/reference/java/sql/Clob.html#getCharacterStream(long,%20long))
        ///
        /// Required features: "java-io-Reader"
        #[cfg(any(feature = "all", all(feature = "java-io-Reader")))]
        pub fn getCharacterStream_long_long<'env>(&'env self, arg0: i64, arg1: i64) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::io::Reader>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Clob", java.flags == PUBLIC | ABSTRACT, .name == "getCharacterStream", .descriptor == "(JJ)Ljava/io/Reader;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Clob\0", "getCharacterStream\0", "(JJ)Ljava/io/Reader;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
