// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-sql-StatementEvent"))]
__jni_bindgen! {
    /// public class [StatementEvent](https://developer.android.com/reference/javax/sql/StatementEvent.html)
    ///
    /// Required feature: javax-sql-StatementEvent
    public class StatementEvent ("javax/sql/StatementEvent") extends crate::java::util::EventObject {

        /// [StatementEvent](https://developer.android.com/reference/javax/sql/StatementEvent.html#StatementEvent(javax.sql.PooledConnection,%20java.sql.PreparedStatement))
        ///
        /// Required features: "java-sql-PreparedStatement", "javax-sql-PooledConnection"
        #[cfg(any(feature = "all", all(feature = "java-sql-PreparedStatement", feature = "javax-sql-PooledConnection")))]
        pub fn new_PooledConnection_PreparedStatement<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::PooledConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::sql::PreparedStatement>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::sql::StatementEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/StatementEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/sql/PooledConnection;Ljava/sql/PreparedStatement;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/StatementEvent\0", "<init>\0", "(Ljavax/sql/PooledConnection;Ljava/sql/PreparedStatement;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [StatementEvent](https://developer.android.com/reference/javax/sql/StatementEvent.html#StatementEvent(javax.sql.PooledConnection,%20java.sql.PreparedStatement,%20java.sql.SQLException))
        ///
        /// Required features: "java-sql-PreparedStatement", "java-sql-SQLException", "javax-sql-PooledConnection"
        #[cfg(any(feature = "all", all(feature = "java-sql-PreparedStatement", feature = "java-sql-SQLException", feature = "javax-sql-PooledConnection")))]
        pub fn new_PooledConnection_PreparedStatement_SQLException<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::PooledConnection>>, arg1: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::sql::PreparedStatement>>, arg2: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::sql::SQLException>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::javax::sql::StatementEvent>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/StatementEvent", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljavax/sql/PooledConnection;Ljava/sql/PreparedStatement;Ljava/sql/SQLException;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into()), __jni_bindgen::AsJValue::as_jvalue(&arg1.into()), __jni_bindgen::AsJValue::as_jvalue(&arg2.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/StatementEvent\0", "<init>\0", "(Ljavax/sql/PooledConnection;Ljava/sql/PreparedStatement;Ljava/sql/SQLException;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getStatement](https://developer.android.com/reference/javax/sql/StatementEvent.html#getStatement())
        ///
        /// Required features: "java-sql-PreparedStatement"
        #[cfg(any(feature = "all", all(feature = "java-sql-PreparedStatement")))]
        pub fn getStatement<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::PreparedStatement>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/StatementEvent", java.flags == PUBLIC, .name == "getStatement", .descriptor == "()Ljava/sql/PreparedStatement;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/StatementEvent\0", "getStatement\0", "()Ljava/sql/PreparedStatement;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [getSQLException](https://developer.android.com/reference/javax/sql/StatementEvent.html#getSQLException())
        ///
        /// Required features: "java-sql-SQLException"
        #[cfg(any(feature = "all", all(feature = "java-sql-SQLException")))]
        pub fn getSQLException<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::SQLException>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/StatementEvent", java.flags == PUBLIC, .name == "getSQLException", .descriptor == "()Ljava/sql/SQLException;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/StatementEvent\0", "getSQLException\0", "()Ljava/sql/SQLException;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
