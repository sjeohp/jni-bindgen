// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-sqlite-SQLiteTransactionListener"))]
__jni_bindgen! {
    /// public interface [SQLiteTransactionListener](https://developer.android.com/reference/android/database/sqlite/SQLiteTransactionListener.html)
    ///
    /// Required feature: android-database-sqlite-SQLiteTransactionListener
    public interface SQLiteTransactionListener ("android/database/sqlite/SQLiteTransactionListener") extends crate::java::lang::Object {

        /// [onBegin](https://developer.android.com/reference/android/database/sqlite/SQLiteTransactionListener.html#onBegin())
        pub fn onBegin<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteTransactionListener", java.flags == PUBLIC | ABSTRACT, .name == "onBegin", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteTransactionListener\0", "onBegin\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onCommit](https://developer.android.com/reference/android/database/sqlite/SQLiteTransactionListener.html#onCommit())
        pub fn onCommit<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteTransactionListener", java.flags == PUBLIC | ABSTRACT, .name == "onCommit", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteTransactionListener\0", "onCommit\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [onRollback](https://developer.android.com/reference/android/database/sqlite/SQLiteTransactionListener.html#onRollback())
        pub fn onRollback<'env>(&'env self) -> __jni_bindgen::std::result::Result<(), __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteTransactionListener", java.flags == PUBLIC | ABSTRACT, .name == "onRollback", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteTransactionListener\0", "onRollback\0", "()V\0");
                __jni_env.call_void_method_a(self.0.object, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
