// WARNING:  This file was autogenerated by jni-bindgen.  Any changes to this file may be lost!!!


#[cfg(any(feature = "all", feature = "android-database-sqlite-SQLiteDoneException"))]
__jni_bindgen! {
    /// public class [SQLiteDoneException](https://developer.android.com/reference/android/database/sqlite/SQLiteDoneException.html)
    ///
    /// Required feature: android-database-sqlite-SQLiteDoneException
    public class SQLiteDoneException ("android/database/sqlite/SQLiteDoneException") extends crate::android::database::sqlite::SQLiteException {

        /// [SQLiteDoneException](https://developer.android.com/reference/android/database/sqlite/SQLiteDoneException.html#SQLiteDoneException())
        pub fn new<'env>(__jni_env: &'env __jni_bindgen::Env) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::sqlite::SQLiteDoneException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteDoneException", java.flags == PUBLIC, .name == "<init>", .descriptor == "()V"
            unsafe {
                let __jni_args = [];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteDoneException\0", "<init>\0", "()V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }

        /// [SQLiteDoneException](https://developer.android.com/reference/android/database/sqlite/SQLiteDoneException.html#SQLiteDoneException(java.lang.String))
        ///
        /// Required features: "java-lang-String"
        #[cfg(any(feature = "all", all(feature = "java-lang-String")))]
        pub fn new_String<'env>(__jni_env: &'env __jni_bindgen::Env, arg0: impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env crate::java::lang::String>>) -> __jni_bindgen::std::result::Result<__jni_bindgen::Local<'env, crate::android::database::sqlite::SQLiteDoneException>, __jni_bindgen::Local<'env, crate::java::lang::Throwable>> {
            // class.path == "android/database/sqlite/SQLiteDoneException", java.flags == PUBLIC, .name == "<init>", .descriptor == "(Ljava/lang/String;)V"
            unsafe {
                let __jni_args = [__jni_bindgen::AsJValue::as_jvalue(&arg0.into())];
                let (__jni_class, __jni_method) = __jni_env.require_class_method("android/database/sqlite/SQLiteDoneException\0", "<init>\0", "(Ljava/lang/String;)V\0");
                __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
            }
        }
    }
}
