// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-sql-Driver"))]
__jni_bindgen! {
    /// public interface [Driver](https://developer.android.com/reference/java/sql/Driver.html)
    ///
    /// Required feature: java-sql-Driver
    public interface Driver ("java/sql/Driver") extends crate::java::lang::Object {

        /// [connect](https://developer.android.com/reference/java/sql/Driver.html#connect(java.lang.String,%20java.util.Properties))
        ///
        /// Required features: "java-lang-String", "java-sql-Connection", "java-util-Properties"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-sql-Connection", feature = "java-util-Properties")))]
        pub fn connect<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Properties>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "connect", .descriptor == "(Ljava/lang/String;Ljava/util/Properties;)Ljava/sql/Connection;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "connect\0", "(Ljava/lang/String;Ljava/util/Properties;)Ljava/sql/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [acceptsURL](https://developer.android.com/reference/java/sql/Driver.html#acceptsURL(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn acceptsURL<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "acceptsURL", .descriptor == "(Ljava/lang/String;)Z"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "acceptsURL\0", "(Ljava/lang/String;)Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getPropertyInfo](https://developer.android.com/reference/java/sql/Driver.html#getPropertyInfo(java.lang.String,%20java.util.Properties))
        ///
        /// Required features: "java-lang-String", "java-sql-DriverPropertyInfo", "java-util-Properties"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-sql-DriverPropertyInfo", feature = "java-util-Properties")))]
        pub fn getPropertyInfo<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::util::Properties>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::sql::DriverPropertyInfo, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "getPropertyInfo", .descriptor == "(Ljava/lang/String;Ljava/util/Properties;)[Ljava/sql/DriverPropertyInfo;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "getPropertyInfo\0", "(Ljava/lang/String;Ljava/util/Properties;)[Ljava/sql/DriverPropertyInfo;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMajorVersion](https://developer.android.com/reference/java/sql/Driver.html#getMajorVersion())
        pub fn getMajorVersion<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "getMajorVersion", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "getMajorVersion\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getMinorVersion](https://developer.android.com/reference/java/sql/Driver.html#getMinorVersion())
        pub fn getMinorVersion<'env>(&'env self) -> __jni_bindgen::std::result::Result<i32, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "getMinorVersion", .descriptor == "()I"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "getMinorVersion\0", "()I\0");
                __jni_env.call_int_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [jdbcCompliant](https://developer.android.com/reference/java/sql/Driver.html#jdbcCompliant())
        pub fn jdbcCompliant<'env>(&'env self) -> __jni_bindgen::std::result::Result<bool, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/Driver", java.flags == PUBLIC | ABSTRACT, .name == "jdbcCompliant", .descriptor == "()Z"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/Driver\0", "jdbcCompliant\0", "()Z\0");
                __jni_env.call_boolean_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
