// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "javax-sql-PooledConnection"))]
__jni_bindgen! {
    /// public interface [PooledConnection](https://developer.android.com/reference/javax/sql/PooledConnection.html)
    ///
    /// Required feature: javax-sql-PooledConnection
    public interface PooledConnection ("javax/sql/PooledConnection") extends crate::java::lang::Object {

        /// [getConnection](https://developer.android.com/reference/javax/sql/PooledConnection.html#getConnection())
        ///
        /// Required features: "java-sql-Connection"
        #[cfg(any(feature = "all", all(feature = "java-sql-Connection")))]
        pub fn getConnection<'env>(&'env self) -> __jni_bindgen::std::result::Result<__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, crate::java::sql::Connection>>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "getConnection", .descriptor == "()Ljava/sql/Connection;"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "getConnection\0", "()Ljava/sql/Connection;\0");
                __jni_env.call_object_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [close](https://developer.android.com/reference/javax/sql/PooledConnection.html#close())
        pub fn close<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "close", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "close\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addConnectionEventListener](https://developer.android.com/reference/javax/sql/PooledConnection.html#addConnectionEventListener(javax.sql.ConnectionEventListener))
        ///
        /// Required features: "javax-sql-ConnectionEventListener"
        #[cfg(any(feature = "all", all(feature = "javax-sql-ConnectionEventListener")))]
        pub fn addConnectionEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::ConnectionEventListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "addConnectionEventListener", .descriptor == "(Ljavax/sql/ConnectionEventListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "addConnectionEventListener\0", "(Ljavax/sql/ConnectionEventListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeConnectionEventListener](https://developer.android.com/reference/javax/sql/PooledConnection.html#removeConnectionEventListener(javax.sql.ConnectionEventListener))
        ///
        /// Required features: "javax-sql-ConnectionEventListener"
        #[cfg(any(feature = "all", all(feature = "javax-sql-ConnectionEventListener")))]
        pub fn removeConnectionEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::ConnectionEventListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "removeConnectionEventListener", .descriptor == "(Ljavax/sql/ConnectionEventListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "removeConnectionEventListener\0", "(Ljavax/sql/ConnectionEventListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [addStatementEventListener](https://developer.android.com/reference/javax/sql/PooledConnection.html#addStatementEventListener(javax.sql.StatementEventListener))
        ///
        /// Required features: "javax-sql-StatementEventListener"
        #[cfg(any(feature = "all", all(feature = "javax-sql-StatementEventListener")))]
        pub fn addStatementEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::StatementEventListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "addStatementEventListener", .descriptor == "(Ljavax/sql/StatementEventListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "addStatementEventListener\0", "(Ljavax/sql/StatementEventListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [removeStatementEventListener](https://developer.android.com/reference/javax/sql/PooledConnection.html#removeStatementEventListener(javax.sql.StatementEventListener))
        ///
        /// Required features: "javax-sql-StatementEventListener"
        #[cfg(any(feature = "all", all(feature = "javax-sql-StatementEventListener")))]
        pub fn removeStatementEventListener<'env>(&'env self, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::javax::sql::StatementEventListener>>) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "javax/sql/PooledConnection", java.flags == PUBLIC | ABSTRACT, .name == "removeStatementEventListener", .descriptor == "(Ljavax/sql/StatementEventListener;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("javax/sql/PooledConnection\0", "removeStatementEventListener\0", "(Ljavax/sql/StatementEventListener;)V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
