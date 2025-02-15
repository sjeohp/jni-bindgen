// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "java-sql-ClientInfoStatus"))]
__jni_bindgen! {
    /// public enum [ClientInfoStatus](https://developer.android.com/reference/java/sql/ClientInfoStatus.html)
    ///
    /// Required feature: java-sql-ClientInfoStatus
    public enum ClientInfoStatus ("java/sql/ClientInfoStatus") extends crate::java::lang::Enum {

        /// [values](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#values())
        ///
        /// Required features: "java-sql-ClientInfoStatus"
        #[cfg(any(feature = "all", all(feature = "java-sql-ClientInfoStatus")))]
        pub fn values<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::sql::ClientInfoStatus, crate::java::lang::Throwable>>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/ClientInfoStatus", java.flags == PUBLIC | STATIC, .name == "values", .descriptor == "()[Ljava/sql/ClientInfoStatus;"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/sql/ClientInfoStatus\0", "values\0", "()[Ljava/sql/ClientInfoStatus;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [valueOf](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#valueOf(java.lang.String))
        ///
        /// Required features: "java-lang-String", "java-sql-ClientInfoStatus"
        #[cfg(any(feature = "all", all(feature = "java-lang-String", feature = "java-sql-ClientInfoStatus")))]
        pub fn valueOf<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "java/sql/ClientInfoStatus", java.flags == PUBLIC | STATIC, .name == "valueOf", .descriptor == "(Ljava/lang/String;)Ljava/sql/ClientInfoStatus;"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_static_method("java/sql/ClientInfoStatus\0", "valueOf\0", "(Ljava/lang/String;)Ljava/sql/ClientInfoStatus;\0");
                __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        // // Not emitting: Non-public method
        // /// [ClientInfoStatus](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#ClientInfoStatus(java.lang.String,%20int))
        // ///
        // /// Required features: "java-lang-String"
        // #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        // fn new<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>, arg1: i32) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
        //     // class.path == "java/sql/ClientInfoStatus", java.flags == PRIVATE, .name == "<init>", .descriptor == "(Ljava/lang/String;I)V"
        //     unsafe {
        //         let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1)];
        //         let (__jni_class, __jni_method) = __jni_env.require_class_method("java/sql/ClientInfoStatus\0", "<init>\0", "(Ljava/lang/String;I)V\0");
        //         __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        //     }
        // }

        /// **get** public static final [REASON_UNKNOWN](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#REASON_UNKNOWN)
        ///
        /// Required feature: java-sql-ClientInfoStatus
        #[cfg(any(feature = "all", feature = "java-sql-ClientInfoStatus"))]
        pub fn REASON_UNKNOWN<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/sql/ClientInfoStatus\0", "REASON_UNKNOWN\0", "Ljava/sql/ClientInfoStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REASON_UNKNOWN_PROPERTY](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#REASON_UNKNOWN_PROPERTY)
        ///
        /// Required feature: java-sql-ClientInfoStatus
        #[cfg(any(feature = "all", feature = "java-sql-ClientInfoStatus"))]
        pub fn REASON_UNKNOWN_PROPERTY<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/sql/ClientInfoStatus\0", "REASON_UNKNOWN_PROPERTY\0", "Ljava/sql/ClientInfoStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REASON_VALUE_INVALID](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#REASON_VALUE_INVALID)
        ///
        /// Required feature: java-sql-ClientInfoStatus
        #[cfg(any(feature = "all", feature = "java-sql-ClientInfoStatus"))]
        pub fn REASON_VALUE_INVALID<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/sql/ClientInfoStatus\0", "REASON_VALUE_INVALID\0", "Ljava/sql/ClientInfoStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        /// **get** public static final [REASON_VALUE_TRUNCATED](https://developer.android.com/reference/java/sql/ClientInfoStatus.html#REASON_VALUE_TRUNCATED)
        ///
        /// Required feature: java-sql-ClientInfoStatus
        #[cfg(any(feature = "all", feature = "java-sql-ClientInfoStatus"))]
        pub fn REASON_VALUE_TRUNCATED<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::ClientInfoStatus>> {
            unsafe {
                let (class, field) = env.require_class_static_field("java/sql/ClientInfoStatus\0", "REASON_VALUE_TRUNCATED\0", "Ljava/sql/ClientInfoStatus;\0");
                env.get_static_object_field(class, field)
            }
        }

        // // Not emitting: Non-public field
        // // Not emitting: Failed to mangle field name: enum $VALUES
        // pub fn get_"$VALUES"<'env>(env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ObjectArray<crate::java::sql::ClientInfoStatus, crate::java::lang::Throwable>>> { ... }
    }
}
